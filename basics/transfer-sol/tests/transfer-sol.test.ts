import { getApi } from '@/clients/shared';
import {
  getTransferSolWithCpiInstruction,
  getTransferSolWithProgramInstruction,
  TRANSFER_SOL_PROGRAM_ADDRESS,
} from '@/clients/transferSol';
import { getCreateAccountInstruction } from '@solana-program/system';
import {
  addSignersToTransactionMessage,
  appendTransactionMessageInstruction,
  appendTransactionMessageInstructions,
  createTransactionMessage,
  generateKeyPairSigner,
  pipe,
  setTransactionMessageFeePayer,
  setTransactionMessageLifetimeUsingBlockhash,
  signTransactionMessageWithSigners,
} from '@solana/kit';
import { test, beforeAll, expect } from 'bun:test';
test('basics:transfer-sol:transfer-sol-with-program', async () => {
  const { defaultPayer, rpc, sendAndConfirmTransaction } = await getApi();
  let { value: latestBlockhash } = await rpc.getLatestBlockhash().send();

  const payerSinger = await generateKeyPairSigner();
  const recipientSinger = await generateKeyPairSigner();

  const transaction = pipe(
    createTransactionMessage({
      version: 0,
    }),
    (tx) => setTransactionMessageFeePayer(defaultPayer.address, tx),
    (tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
    (tx) =>
      appendTransactionMessageInstructions(
        [
          getCreateAccountInstruction({
            newAccount: payerSinger,
            payer: defaultPayer,
            space: 0,
            lamports: 1000000, // 0.001 SOL
            programAddress: TRANSFER_SOL_PROGRAM_ADDRESS,
          }),
          getTransferSolWithProgramInstruction({
            payer: payerSinger,
            recipient: recipientSinger.address,
            amount: 1000000, // 0.001 SOL
          }),
        ],
        tx
      ),
    (tx) => addSignersToTransactionMessage([defaultPayer, payerSinger], tx)
  );
  const signedTransaction =
    await signTransactionMessageWithSigners(transaction);

  await sendAndConfirmTransaction(signedTransaction, {
    commitment: 'confirmed',
  });
});

test('basics:transfer-sol:transfer-sol-with-cpi', async () => {
  const { defaultPayer, rpc, sendAndConfirmTransaction } = await getApi();
  let { value: latestBlockhash } = await rpc.getLatestBlockhash().send();

  const recipientSinger = await generateKeyPairSigner();

  const transaction = pipe(
    createTransactionMessage({
      version: 0,
    }),
    (tx) => setTransactionMessageFeePayer(defaultPayer.address, tx),
    (tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
    (tx) =>
      appendTransactionMessageInstruction(
        getTransferSolWithCpiInstruction({
          payer: defaultPayer,
          recipient: recipientSinger.address,
          amount: 10000000, // 0.0001 SOL
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
