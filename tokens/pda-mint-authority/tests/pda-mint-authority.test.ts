import {
  getCreateTokenInstruction,
  getInitMintAuthorityInstruction,
  PDA_MINT_AUTHORITY_PROGRAM_ADDRESS,
} from '@/clients/pdaMintAuthority';
import { getApi } from '@/clients/shared';
import {
  addSignersToTransactionMessage,
  appendTransactionMessageInstruction,
  createTransactionMessage,
  generateKeyPairSigner,
  getAddressEncoder,
  getProgramDerivedAddress,
  pipe,
  setTransactionMessageFeePayer,
  setTransactionMessageLifetimeUsingBlockhash,
  signTransactionMessageWithSigners,
} from '@solana/kit';
import { expect, test, beforeAll, afterAll } from 'bun:test';

let mintKeypair: Awaited<ReturnType<typeof generateKeyPairSigner>>;

beforeAll(async () => {
  mintKeypair = await generateKeyPairSigner();
});

test('tokens:pda-mint-authority:init-mint-authority', async () => {
  const addressEncoder = getAddressEncoder();
  const { defaultPayer, rpc, sendAndConfirmTransaction } = await getApi();
  let { value: latestBlockhash } = await rpc.getLatestBlockhash().send();

  const [mintAuthorityPubkey, bump] = await getProgramDerivedAddress({
    programAddress: PDA_MINT_AUTHORITY_PROGRAM_ADDRESS,
    seeds: [Buffer.from('mint_authority')],
  });

  const transaction = pipe(
    createTransactionMessage({
      version: 0,
    }),
    (tx) => setTransactionMessageFeePayer(defaultPayer.address, tx),
    (tx) => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
    (tx) =>
      appendTransactionMessageInstruction(
        getInitMintAuthorityInstruction({
          payer: defaultPayer,
          mintAuthority: mintAuthorityPubkey,
          bump,
        }),
        tx
      ),
    (tx) => addSignersToTransactionMessage([defaultPayer], tx)
  );

  const signedTransactionMintNft =
    await signTransactionMessageWithSigners(transaction);

  await sendAndConfirmTransaction(signedTransactionMintNft, {
    commitment: 'confirmed',
  });
});

test.skip('tokens:pda-mint-authority:create-token', async () => {
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

  const signedTransactionMintNft =
    await signTransactionMessageWithSigners(transaction);

  await sendAndConfirmTransaction(signedTransactionMintNft, {
    commitment: 'confirmed',
  });
});
