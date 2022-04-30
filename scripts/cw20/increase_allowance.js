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

const wallet = wallets.myKeyWallet;
const spenderWallet = wallets.gatchaWallet;

const msg = new MsgExecuteContract(
    // Address of wallet that is signing the transaction
    wallet.key.accAddress,
    // Address of CW20 contract
    cw20Contract,
    // ExecuteMsg payload
    {
        increase_allowance: {
          // Address of wallet or contract that is getting the tokens
          spender: spenderWallet.key.accAddress,
          // Amount of tokens to increase allowance, in microunits
          amount: "100000000",
          expires: {
              at_time: "1653941444000",
          },
        },
    },
  );

  const tx = await wallet.createAndSignTx({ msgs: [msg] });
  const result = await client.tx.broadcast(tx);

console.log(result);