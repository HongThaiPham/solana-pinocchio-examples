import {
  address,
  createKeyPairSignerFromBytes,
  createSolanaRpc,
  createSolanaRpcSubscriptions,
  sendAndConfirmTransactionFactory,
} from '@solana/kit';

import PAYER_PRIVATE_KEY from '/home/leo/.config/solana/id.json';
export const RPC_HOST = 'api.devnet.solana.com';

export const getApi = async () => {
  const defaultPayer = await createKeyPairSignerFromBytes(
    new Uint8Array(PAYER_PRIVATE_KEY)
  );

  const rpc = createSolanaRpc(`https://${RPC_HOST}`);
  const rpcSubscriptions = createSolanaRpcSubscriptions(`wss://${RPC_HOST}`);

  const sendAndConfirmTransaction = sendAndConfirmTransactionFactory({
    /**
     * The RPC implements a `sendTransaction` method which relays transactions to the network.
     */
    rpc,
    /**
     * RPC subscriptions allow the transaction sender to subscribe to the status of our transaction.
     * The sender will resolve when the transaction is reported to have been confirmed, or will
     * reject in the event of an error, or a timeout if the transaction lifetime is thought to have
     * expired.
     */
    rpcSubscriptions,
  });

  return {
    defaultPayer,
    rpc,
    rpcSubscriptions,
    sendAndConfirmTransaction,
  };
};

export const explorerUrl = (tx: string) =>
  `https://explorer.solana.com/tx/${tx}?cluster=devnet`;

export const SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ADDRESS = address(
  'ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL'
);
