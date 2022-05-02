import { client, wallets } from '../library.js';

import {
  MsgExecuteContract,
  Coins,
} from "@terra-money/terra.js";

// const contract = "terra1f7vnhhk24jmaxp2tdgf906yy980yg3c4h8f05h";
// testnet
// swap contract address of the Gacha token
const contract = "terra1v85ctl8fg8097xx93musqgl0wg99lx6kwk8fhq";
const wallet = wallets.bombayWallet;

const amount = (0.1 * 1e6).toFixed(0);

const msg = new MsgExecuteContract(
  // Address of person who's signing the transaction.
  wallet.key.accAddress,
  // Address of contract to execute.
  contract,
  // ExecuteMsg payload
  {
    buy: {},
  },
  // Send Luna with this execute message.
  new Coins({ uluna: amount }),
);

const tx = await wallet.createAndSignTx({ msgs: [msg] });
const result = await client.tx.broadcast(tx);

console.log(result);