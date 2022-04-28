import { client, wallets } from '../library.js';

import {
  MsgExecuteContract,
  Coins,
} from "@terra-money/terra.js";

const contract = "terra1pcknsatx5ceyfu6zvtmz3yr8auumzrdts4ax4a";
const wallet = wallets.test1Wallet;

const newPrice = "300000";

const msg = new MsgExecuteContract(
    wallet.key.accAddress,
    contract,
    {
        update_price: { new_price: newPrice },
    },
);

const tx = await wallet.createAndSignTx({ msgs: [msg] });
const result = await client.tx.broadcast(tx);

console.log(result);