import {
  CLOSE_ACCOUNT_PROGRAM_ADDRESS,
  fetchUser,
  getCloseUserInstruction,
  getCreateUserInstruction,
} from '@/clients/closeAccount';
import { getApi } from '@/clients/shared';

import {
  getCreateAccountInstruction,
  getTransferSolInstruction,
} from '@solana-program/system';
import {
  addSignersToTransactionMessage,
  appendTransactionMessageInstruction,
  appendTransactionMessageInstructions,
  createTransactionMessage,
  generateKeyPairSigner,
  getAddressEncoder,
  getProgramDerivedAddress,
  pipe,
  setTransactionMessageFeePayer,
  setTransactionMessageLifetimeUsingBlockhash,
  signTransactionMessageWithSigners,
  type KeyPairSigner,
} from '@solana/kit';
import { test, expect, beforeAll } from 'bun:test';
let userSinger: KeyPairSigner;

beforeAll(async () => {
  const { defaultPayer, rpc, sendAndConfirmTransaction } = await getApi();
  let { value: latestBlockhash } = await rpc.getLatestBlockhash().send();

  userSinger = await generateKeyPairSigner();

  const transaction = pipe(
    createTransactionMessage({
      version: 0,
    }),
    (tx) => setTransactionMessageFeePayer(defaultPayer.address, tx),
    (tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
    (tx) =>
      appendTransactionMessageInstruction(
        getTransferSolInstruction({
          source: defaultPayer,
          destination: userSinger.address,
          amount: 2500000, // 0.0025 SOL
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

test('basics:close-account:create-user', async () => {
  const addressEncoder = getAddressEncoder();

  const { defaultPayer, rpc, sendAndConfirmTransaction } = await getApi();
  let { value: latestBlockhash } = await rpc.getLatestBlockhash().send();

  const [targetAccountPubkey, bump] = await getProgramDerivedAddress({
    programAddress: CLOSE_ACCOUNT_PROGRAM_ADDRESS,
    seeds: [Buffer.from('USER'), addressEncoder.encode(userSinger.address)],
  });

  const transaction = pipe(
    createTransactionMessage({
      version: 0,
    }),
    (tx) => setTransactionMessageFeePayer(defaultPayer.address, tx),
    (tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
    (tx) =>
      appendTransactionMessageInstruction(
        getCreateUserInstruction({
          payer: userSinger,
          targetAccount: targetAccountPubkey,
          name: 'Alice',
        }),

        tx
      ),
    (tx) => addSignersToTransactionMessage([defaultPayer, userSinger], tx)
  );
  const signedTransaction =
    await signTransactionMessageWithSigners(transaction);

  await sendAndConfirmTransaction(signedTransaction, {
    commitment: 'confirmed',
  });

  const targetAccount = await fetchUser(rpc, targetAccountPubkey);
  expect(targetAccount).toBeDefined();
  expect(targetAccount?.data.name).toBe('Alice');
});

test('basics:close-account:close-user', async () => {
  const addressEncoder = getAddressEncoder();

  const { defaultPayer, rpc, sendAndConfirmTransaction } = await getApi();
  let { value: latestBlockhash } = await rpc.getLatestBlockhash().send();

  const [targetAccountPubkey, bump] = await getProgramDerivedAddress({
    programAddress: CLOSE_ACCOUNT_PROGRAM_ADDRESS,
    seeds: [Buffer.from('USER'), addressEncoder.encode(userSinger.address)],
  });

  const transaction = pipe(
    createTransactionMessage({
      version: 0,
    }),
    (tx) => setTransactionMessageFeePayer(defaultPayer.address, tx),
    (tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
    (tx) =>
      appendTransactionMessageInstruction(
        getCloseUserInstruction({
          payer: userSinger,
          targetAccount: targetAccountPubkey,
        }),
        tx
      ),
    (tx) => addSignersToTransactionMessage([defaultPayer, userSinger], tx)
  );
  const signedTransaction =
    await signTransactionMessageWithSigners(transaction);

  await sendAndConfirmTransaction(signedTransaction, {
    commitment: 'confirmed',
  });

  try {
    await fetchUser(rpc, targetAccountPubkey);
  } catch (error) {
    expect((error as Error).message).toContain(
      `Account not found at address: ${targetAccountPubkey}`
    );
  }
});
