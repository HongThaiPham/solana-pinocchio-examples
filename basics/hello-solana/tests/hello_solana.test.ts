import { test } from 'bun:test';
import { getApi } from '@clients/shared';
import {
  addSignersToTransactionMessage,
  appendTransactionMessageInstruction,
  createTransactionMessage,
  pipe,
  setTransactionMessageFeePayer,
  setTransactionMessageLifetimeUsingBlockhash,
  signTransactionMessageWithSigners,
} from '@solana/kit';
import { getHelloInstruction } from '@clients/hello-solana';

test('basics:hello-solana', async () => {
  const { defaultPayer, rpc, sendAndConfirmTransaction } = await getApi();
  let { value: latestBlockhash } = await rpc.getLatestBlockhash().send();

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
});
