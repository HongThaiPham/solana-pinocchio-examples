import { getApi } from '@/clients/shared';
import {
  getMintTokenInstruction,
  getCreateTokenInstruction,
} from '@/clients/splTokenMinter';
import {
  addSignersToTransactionMessage,
  appendTransactionMessageInstruction,
  createTransactionMessage,
  generateKeyPairSigner,
  getAddressEncoder,
  pipe,
  setTransactionMessageFeePayer,
  setTransactionMessageLifetimeUsingBlockhash,
  signTransactionMessageWithSigners,
} from '@solana/kit';
import { test, beforeAll, expect } from 'bun:test';

import {
  TOKEN_PROGRAM_ADDRESS,
  findAssociatedTokenPda,
} from '@solana-program/token';

let mintKeypair: Awaited<ReturnType<typeof generateKeyPairSigner>>;
beforeAll(async () => {
  mintKeypair = await generateKeyPairSigner();
});

test('basics:spl-token-minter:createToken', async () => {
  const addressEncoder = getAddressEncoder();
  const { defaultPayer, rpc, sendAndConfirmTransaction } = await getApi();
  let { value: latestBlockhash } = await rpc.getLatestBlockhash().send();

  const transaction = pipe(
    createTransactionMessage({
      version: 0,
    }),
    (tx) => setTransactionMessageFeePayer(defaultPayer.address, tx),
    (tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
    (tx) =>
      appendTransactionMessageInstruction(
        getCreateTokenInstruction({
          payer: defaultPayer,
          tokenDecimals: 9,
          mint: mintKeypair,
          mintAuthority: defaultPayer.address,
          freezeAuthority: defaultPayer.address,
        }),
        tx
      ),
    (tx) => addSignersToTransactionMessage([defaultPayer], tx)
  );

  const signedTransaction =
    await signTransactionMessageWithSigners(transaction);

  await sendAndConfirmTransaction(signedTransaction, {
    commitment: 'confirmed',
  });
});

test('basics:spl-token-minter:mintToken', async () => {
  const addressEncoder = getAddressEncoder();
  const { defaultPayer, rpc, sendAndConfirmTransaction } = await getApi();
  let { value: latestBlockhash } = await rpc.getLatestBlockhash().send();

  const receiverAddress = await generateKeyPairSigner();

  const [tokenAccount] = await findAssociatedTokenPda({
    mint: mintKeypair.address,
    owner: receiverAddress.address,
    tokenProgram: TOKEN_PROGRAM_ADDRESS,
  });

  const transaction = pipe(
    createTransactionMessage({
      version: 0,
    }),
    (tx) => setTransactionMessageFeePayer(defaultPayer.address, tx),
    (tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
    (tx) =>
      appendTransactionMessageInstruction(
        getMintTokenInstruction({
          amount: 100 * 10 ** 9, // 100 tokens with 9 decimals
          mint: mintKeypair.address,
          mintAuthority: defaultPayer,
          to: receiverAddress.address,
          tokenAccount,
        }),
        tx
      ),
    (tx) => addSignersToTransactionMessage([defaultPayer], tx)
  );

  const signedTransaction =
    await signTransactionMessageWithSigners(transaction);

  await sendAndConfirmTransaction(signedTransaction, {
    commitment: 'confirmed',
  });

  const tokenAccountBalance = await rpc
    .getTokenAccountBalance(tokenAccount)
    .send();

  expect(tokenAccountBalance.value.uiAmountString.toString()).toBe('100');
});
