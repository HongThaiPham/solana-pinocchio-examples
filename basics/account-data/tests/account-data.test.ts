import { expect, test } from 'bun:test';
import { getApi } from '@clients/shared';
import {
  addSignersToTransactionMessage,
  appendTransactionMessageInstruction,
  createTransactionMessage,
  pipe,
  setTransactionMessageFeePayer,
  setTransactionMessageLifetimeUsingBlockhash,
  signTransactionMessageWithSigners,
  generateKeyPairSigner,
} from '@solana/kit';
import { fetchAddressInfo, getCreateInstruction } from '@clients/accountData';

test('basics:account-data:create', async () => {
  const { defaultPayer, rpc, sendAndConfirmTransaction } = await getApi();
  let { value: latestBlockhash } = await rpc.getLatestBlockhash().send();

  const addressInfoKeypair = await generateKeyPairSigner();

  const transactionMintNftMessage = pipe(
    createTransactionMessage({
      version: 0,
    }),
    (tx) => setTransactionMessageFeePayer(defaultPayer.address, tx),
    (tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
    (tx) =>
      appendTransactionMessageInstruction(
        getCreateInstruction({
          addressInfo: addressInfoKeypair,
          owner: defaultPayer,
          name: 'John Doe',
          houseNumber: 123,
          street: 'Main St',
          city: 'Anytown',
        }),
        tx
      ),
    (tx) => addSignersToTransactionMessage([defaultPayer], tx)
  );
  const signedTransactionMintNft = await signTransactionMessageWithSigners(
    transactionMintNftMessage
  );

  await sendAndConfirmTransaction(signedTransactionMintNft, {
    commitment: 'confirmed',
  });

  const { data } = await fetchAddressInfo(rpc, addressInfoKeypair.address);
  console.log('Address Info:', data);

  expect(data).toEqual({
    name: 'John Doe',
    houseNumber: 123,
    street: 'Main St',
    city: 'Anytown',
  });
});
