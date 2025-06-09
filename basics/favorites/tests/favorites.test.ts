import { expect, test, beforeAll, afterAll } from 'bun:test';
import { getApi } from '@clients/shared';
import {
  addSignersToTransactionMessage,
  createTransactionMessage,
  pipe,
  setTransactionMessageFeePayer,
  setTransactionMessageLifetimeUsingBlockhash,
  signTransactionMessageWithSigners,
  generateKeyPairSigner,
  getProgramDerivedAddress,
  getAddressEncoder,
  appendTransactionMessageInstructions,
  appendTransactionMessageInstruction,
} from '@solana/kit';
import {
  FAVORITES_PROGRAM_ADDRESS,
  fetchFavorites,
  getCreatePdaInstruction,
  getGetPdaInstruction,
} from '@clients/favorites';
import { getTransferSolInstruction } from '@solana-program/system';
let newUser: Awaited<ReturnType<typeof generateKeyPairSigner>>;
beforeAll(async () => {
  newUser = await generateKeyPairSigner();
});

test('basics:favorites:createPda', async () => {
  const addressEncoder = getAddressEncoder();
  const { defaultPayer, rpc, sendAndConfirmTransaction } = await getApi();
  let { value: latestBlockhash } = await rpc.getLatestBlockhash().send();

  const [favoritesPubkey, bump] = await getProgramDerivedAddress({
    programAddress: FAVORITES_PROGRAM_ADDRESS,
    seeds: [Buffer.from('favorites'), addressEncoder.encode(newUser.address)],
  });

  const transactionMintNftMessage = pipe(
    createTransactionMessage({
      version: 0,
    }),
    (tx) => setTransactionMessageFeePayer(defaultPayer.address, tx),
    (tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
    (tx) =>
      appendTransactionMessageInstructions(
        [
          getTransferSolInstruction({
            source: defaultPayer,
            destination: newUser.address,
            amount: 4000000, // 0.004 SOL
          }),
          getCreatePdaInstruction({
            user: newUser,
            favorites: favoritesPubkey,
            number: '123',
            color: 'blue',
            hobbies: ['reading', 'gaming', 'coding', 'hiking', 'cooking'],
            bump,
          }),
        ],
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

  const { data } = await fetchFavorites(rpc, favoritesPubkey);

  expect(data).toBeDefined();
  expect(data).toEqual({
    number: '123',
    color: 'blue',
    hobbies: ['reading', 'gaming', 'coding', 'hiking', 'cooking'],
    bump,
  });
});

test('basics:favorites:getPda', async () => {
  const addressEncoder = getAddressEncoder();
  const { defaultPayer, rpc, sendAndConfirmTransaction } = await getApi();
  let { value: latestBlockhash } = await rpc.getLatestBlockhash().send();

  const [favoritesPubkey] = await getProgramDerivedAddress({
    programAddress: FAVORITES_PROGRAM_ADDRESS,
    seeds: [Buffer.from('favorites'), addressEncoder.encode(newUser.address)],
  });

  const transactionMintNftMessage = pipe(
    createTransactionMessage({
      version: 0,
    }),
    (tx) => setTransactionMessageFeePayer(defaultPayer.address, tx),
    (tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
    (tx) =>
      appendTransactionMessageInstruction(
        getGetPdaInstruction({
          user: newUser,
          favorites: favoritesPubkey,
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
});

afterAll(async () => {
  // Cleanup if necessary
  const { defaultPayer, rpc, sendAndConfirmTransaction } = await getApi();
  let { value: latestBlockhash } = await rpc.getLatestBlockhash().send();

  const remainingFee = await rpc.getBalance(newUser.address).send();

  const collectFee = pipe(
    createTransactionMessage({
      version: 0,
    }),
    (tx) => setTransactionMessageFeePayer(defaultPayer.address, tx),
    (tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
    (tx) =>
      appendTransactionMessageInstructions(
        [
          getTransferSolInstruction({
            source: newUser,
            destination: defaultPayer.address,
            amount: remainingFee.value,
          }),
        ],
        tx
      ),
    (tx) => addSignersToTransactionMessage([defaultPayer, newUser], tx)
  );
  const signedTransactionCollectFee =
    await signTransactionMessageWithSigners(collectFee);

  await sendAndConfirmTransaction(signedTransactionCollectFee, {
    commitment: 'confirmed',
  });
});
