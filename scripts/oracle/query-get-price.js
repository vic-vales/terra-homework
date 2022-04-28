import { client, wallets } from '../library.js';

const contract = "terra1pcknsatx5ceyfu6zvtmz3yr8auumzrdts4ax4a";

const response = await client.wasm.contractQuery(contract, { get_price: {} });

console.log(response);