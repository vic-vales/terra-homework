import { client, wallets } from '../library.js';

import {
  MsgExecuteContract,
  MnemonicKey,
  Coins,
  LCDClient,
} from "@terra-money/terra.js";

// const cw20Contract = "terra1hpajld8zs93md8zrs6sfy42zl0khqpmr07muw0";
const cw20Contract = "terra1fpud4qznh6p0r7qg3mcfs3semz48g5228esct2";
const walletAddress = wallets.myKeyWallet.key.accAddress;

const response = await client.wasm.contractQuery(
  // Address of CW20 contract.
  cw20Contract,
  // QueryMsg payload.
  {
    balance: {
      address: walletAddress
    }
  }
);

console.log(response);

