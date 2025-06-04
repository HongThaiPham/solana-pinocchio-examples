import { expect, test } from 'bun:test';
import { getApi } from '@clients/shared';
import {
  addSignersToTransactionMessage,
  appendTransactionMessageInstruction,
  createTransactionMessage,
  getProgramDerivedAddress,
  pipe,
  setTransactionMessageFeePayer,
  setTransactionMessageLifetimeUsingBlockhash,
  signTransactionMessageWithSigners,
} from '@solana/kit';
import {
  COUNTER_PROGRAM_ADDRESS,
  fetchCounter,
  getCreateInstruction,
  getDecreaseInstruction,
  getIncreaseInstruction,
} from '@clients/counter';

const [counterPubkey, bump] = await getProgramDerivedAddress({
  programAddress: COUNTER_PROGRAM_ADDRESS,
  seeds: [Buffer.from('counter')],
});

const isInitialized = await (async () => {
  const { rpc } = await getApi();
  try {
    const { data } = await fetchCounter(rpc, counterPubkey);
    return !!data.count;
  } catch (error) {
    return false;
  }
})();

test.skipIf(isInitialized)('basics:counter:create', async () => {
  const { defaultPayer, rpc, sendAndConfirmTransaction } = await getApi();
  let { value: latestBlockhash } = await rpc.getLatestBlockhash().send();

  const transactionMintNftMessage = pipe(
    createTransactionMessage({
      version: 0,
    }),
    (tx) => setTransactionMessageFeePayer(defaultPayer.address, tx),
    (tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
    (tx) =>
      appendTransactionMessageInstruction(
        getCreateInstruction({
          counter: counterPubkey,
          initialValue: 0n,
          bump,
          owner: defaultPayer,
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

  const { data } = await fetchCounter(rpc, counterPubkey);
  expect(data.count).toBe(1n);
});

test('basics:counter:incease', async () => {
  const { defaultPayer, rpc, sendAndConfirmTransaction } = await getApi();

  const { data: dataBefore } = await fetchCounter(rpc, counterPubkey);
  let { value: latestBlockhash } = await rpc.getLatestBlockhash().send();

  const transactionMintNftMessage = pipe(
    createTransactionMessage({
      version: 0,
    }),
    (tx) => setTransactionMessageFeePayer(defaultPayer.address, tx),
    (tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
    (tx) =>
      appendTransactionMessageInstruction(
        getIncreaseInstruction({
          counter: counterPubkey,
          owner: defaultPayer,
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

  const { data } = await fetchCounter(rpc, counterPubkey);
  expect(data.count).toBe(dataBefore.count + 1n);
});

test('basics:counter:decrease', async () => {
  const { defaultPayer, rpc, sendAndConfirmTransaction } = await getApi();

  const { data: dataBefore } = await fetchCounter(rpc, counterPubkey);
  let { value: latestBlockhash } = await rpc.getLatestBlockhash().send();

  const transactionMintNftMessage = pipe(
    createTransactionMessage({
      version: 0,
    }),
    (tx) => setTransactionMessageFeePayer(defaultPayer.address, tx),
    (tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
    (tx) =>
      appendTransactionMessageInstruction(
        getDecreaseInstruction({
          counter: counterPubkey,
          owner: defaultPayer,
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

  const { data } = await fetchCounter(rpc, counterPubkey);
  expect(data.count).toBe(dataBefore.count - 1n);
});
