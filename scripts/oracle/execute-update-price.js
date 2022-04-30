import { client, wallets } from '../library.js';

import {
  MsgExecuteContract,
  Coins,
} from "@terra-money/terra.js";

// localterra
// const contract = "terra1pcknsatx5ceyfu6zvtmz3yr8auumzrdts4ax4a";
// const wallet = wallets.test1Wallet;

// testnet
const contract = "terra1e3pgyrxujulm067376ldz5mvvyaexx60lvc9dh";
const wallet = wallets.gatchaWallet;

const newPrice = "250000";

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