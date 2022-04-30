import { client, wallets } from '../library.js';

// local
// const contract = "terra1pcknsatx5ceyfu6zvtmz3yr8auumzrdts4ax4a";

// testnet
const contract = "terra1e3pgyrxujulm067376ldz5mvvyaexx60lvc9dh";

const response = await client.wasm.contractQuery(contract, { get_price: {} });

console.log(response);