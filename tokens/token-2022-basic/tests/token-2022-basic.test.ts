import { getApi } from '@/clients/shared';
import {
  getMintTokenInstruction,
  getCreateTokenInstruction,
  getTransferInstruction,
} from '@/clients/token2022Basic';
import {
  addSignersToTransactionMessage,
  appendTransactionMessageInstruction,
  appendTransactionMessageInstructions,
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
  TOKEN_2022_PROGRAM_ADDRESS,
  findAssociatedTokenPda,
} from '@solana-program/token-2022';

let mintKeypair: Awaited<ReturnType<typeof generateKeyPairSigner>>;
beforeAll(async () => {
  mintKeypair = await generateKeyPairSigner();
});

test('tokens:token-2022-basic:createToken', async () => {
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
          tokenName: 'Solana Token',
          tokenDecimals: 9,
          mint: mintKeypair,
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

test('tokens:token-2022-basic:mintToken', async () => {
  const addressEncoder = getAddressEncoder();
  const { defaultPayer, rpc, sendAndConfirmTransaction } = await getApi();
  let { value: latestBlockhash } = await rpc.getLatestBlockhash().send();

  const receiverAddress = await generateKeyPairSigner();

  const [tokenAccount] = await findAssociatedTokenPda({
    mint: mintKeypair.address,
    owner: receiverAddress.address,
    tokenProgram: TOKEN_2022_PROGRAM_ADDRESS,
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

test('tokens:token-2022-basic:transfer', async () => {
  const addressEncoder = getAddressEncoder();
  const { defaultPayer, rpc, sendAndConfirmTransaction } = await getApi();
  let { value: latestBlockhash } = await rpc.getLatestBlockhash().send();

  const receiverAddress = await generateKeyPairSigner();

  const [toTokenAccount] = await findAssociatedTokenPda({
    mint: mintKeypair.address,
    owner: receiverAddress.address,
    tokenProgram: TOKEN_2022_PROGRAM_ADDRESS,
  });

  const [fromTokenAccount] = await findAssociatedTokenPda({
    mint: mintKeypair.address,
    owner: defaultPayer.address,
    tokenProgram: TOKEN_2022_PROGRAM_ADDRESS,
  });

  const transaction = pipe(
    createTransactionMessage({
      version: 0,
    }),
    (tx) => setTransactionMessageFeePayer(defaultPayer.address, tx),
    (tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
    (tx) =>
      appendTransactionMessageInstructions(
        [
          getMintTokenInstruction({
            amount: 100 * 10 ** 9, // 100 tokens with 9 decimals
            mint: mintKeypair.address,
            mintAuthority: defaultPayer,
            to: defaultPayer.address,
            tokenAccount: fromTokenAccount,
          }),
          getTransferInstruction({
            amount: 100 * 10 ** 9, // 100 tokens with 9 decimals
            mint: mintKeypair.address,
            from: defaultPayer,
            to: receiverAddress.address,
            toTokenAccount: toTokenAccount,
            fromTokenAccount: fromTokenAccount,
          }),
        ],
        tx
      ),
    (tx) => addSignersToTransactionMessage([defaultPayer], tx)
  );

  const signedTransaction =
    await signTransactionMessageWithSigners(transaction);

  await sendAndConfirmTransaction(signedTransaction, {
    commitment: 'confirmed',
  });

  const fromTokenAccountBalance = await rpc
    .getTokenAccountBalance(fromTokenAccount)
    .send();

  expect(fromTokenAccountBalance.value.uiAmountString.toString()).toBe('0');

  const toTokenAccountBalance = await rpc
    .getTokenAccountBalance(toTokenAccount)
    .send();

  expect(toTokenAccountBalance.value.uiAmountString.toString()).toBe('100');
});
