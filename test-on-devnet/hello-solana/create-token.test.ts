import { expect, test } from 'bun:test';
import { getApi } from '../shared/lib';
import {
  addSignersToTransactionMessage,
  appendTransactionMessageInstruction,
  createTransactionMessage,
  pipe,
  setTransactionMessageFeePayer,
  setTransactionMessageLifetimeUsingBlockhash,
  signTransactionMessageWithSigners,
} from '@solana/kit';
import { getHelloInstruction } from '../../clients/hello-solana';

test('tokens:create-token', async () => {
  const { defaultPayer, rpc, sendAndConfirmTransaction } = await getApi();
  let { value: latestBlockhash } = await rpc.getLatestBlockhash().send();
  console.log(defaultPayer.address);

  const transactionMintNftMessage = pipe(
    createTransactionMessage({
      version: 0,
    }),
    (tx) => setTransactionMessageFeePayer(defaultPayer.address, tx),
    (tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
    (tx) => appendTransactionMessageInstruction(getHelloInstruction(), tx),
    (tx) => addSignersToTransactionMessage([defaultPayer], tx)
  );
  const signedTransactionMintNft = await signTransactionMessageWithSigners(
    transactionMintNftMessage
  );

  await sendAndConfirmTransaction(signedTransactionMintNft, {
    commitment: 'confirmed',
  });

  expect(2 + 2).toBe(4);
});
