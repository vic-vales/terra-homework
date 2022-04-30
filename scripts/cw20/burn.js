import { client, wallets } from '../library.js';

import {
  MsgExecuteContract,
  MnemonicKey,
  Coins,
  LCDClient,
} from "@terra-money/terra.js";

// const cw20Contract = "terra1hpajld8zs93md8zrs6sfy42zl0khqpmr07muw0";

// testnet contract
const cw20Contract = "terra1fpud4qznh6p0r7qg3mcfs3semz48g5228esct2";
const wallet = wallets.gatchaWallet;
const recipientWallet = wallets.myKeyWallet;

const msg = new MsgExecuteContract(
    // Address of wallet that is signing the transaction
    wallet.key.accAddress,
    // Address of CW20 contract
    cw20Contract,
    // ExecuteMsg payload
    {
        burn: {
          // Address of wallet or contract that is getting the tokens
          // recipient: recipientWallet.key.accAddress,
          // Amount of tokens to transfer, in microunits
          amount: "10000000000",
        },
    },
  );

  const tx = await wallet.createAndSignTx({ msgs: [msg] });
  const result = await client.tx.broadcast(tx);

console.log(result);