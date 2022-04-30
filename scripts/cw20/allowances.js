import { client, wallets } from '../library.js';

import {
  MsgExecuteContract,
  MnemonicKey,
  Coins,
  LCDClient,
} from "@terra-money/terra.js";

// const cw20Contract = "terra1hpajld8zs93md8zrs6sfy42zl0khqpmr07muw0";
const cw20Contract = "terra1fpud4qznh6p0r7qg3mcfs3semz48g5228esct2";

const ownerWalletAddress = wallets.myKeyWallet.key.accAddress;
const spenderWalletAddress = wallets.gatchaWallet.key.accAddress;


let response = await client.wasm.contractQuery(cw20Contract, { all_allowances: { owner: ownerWalletAddress }});
console.log(`All allowances`, response);

response = await client.wasm.contractQuery(cw20Contract, { allowance: { owner: ownerWalletAddress, spender: spenderWalletAddress }});
console.log(`Allowance`, response);

