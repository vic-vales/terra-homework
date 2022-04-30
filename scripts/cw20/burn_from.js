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
const ownerWallet = wallets.myKeyWallet;

const msg = new MsgExecuteContract(
    // Address of wallet that is signing the transaction
    wallet.key.accAddress,
    // Address of CW20 contract
    cw20Contract,
    // ExecuteMsg payload
    {
        burn_from: {
          // Address of wallet or contract to burn tokens from
          owner: ownerWallet.key.accAddress,
          // Amount of tokens to burn, in microunits
          amount: "460000000",
        },
    },
  );

  const tx = await wallet.createAndSignTx({ msgs: [msg] });
  const result = await client.tx.broadcast(tx);

console.log(result);