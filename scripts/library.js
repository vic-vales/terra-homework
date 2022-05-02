import fetch from 'isomorphic-fetch';
import { Coins, LCDClient } from '@terra-money/terra.js';
const gasPrices =  await fetch('https://bombay-fcd.terra.dev/v1/txs/gas_prices');
const gasPricesJson = await gasPrices.json();

// LCD stands for "Light Client Daemon". I don't really know much about it, but
// this is how you talk to Terra from JS.
const client = new LCDClient({
  URL: "https://bombay-lcd.terra.dev/", // Use "https://lcd.terra.dev" for prod "http://localhost:1317" for localterra.
  chainID: "bombay-12", // Use "columbus-5" for production or "localterra".
  gasPrices: { uluna: gasPricesJson['uluna'] },
  gasAdjustment: "1.5", // Increase gas price slightly so transactions go through smoothly.
  gas: 10000000,
});

// const client = new LCDClient({
//   URL: "http://localhost:1317",
//   chainID: "localterra", // Use "columbus-5" for production or "localterra".
//   gasPrices: { uluna: gasPricesJson['uluna'] },
//   gasAdjustment: "1.5", // Increase gas price slightly so transactions go through smoothly.
//   gas: 10000000,
// });


import { MnemonicKey } from '@terra-money/terra.js';

const wallets = {
  wallet1: client.wallet(new MnemonicKey({
    mnemonic: "confirm electric wink vocal nut flat globe machine gown million develop quiz dune bar coil favorite need skin iron husband mutual shoulder depth today",
  })),
  wallet2: client.wallet(new MnemonicKey({
    mnemonic: "warm lucky circle bicycle quote lemon omit one robot include fruit fix coach parrot identify glance foil random fox tornado diagram twist flat picnic",
  })),
  gatchaWallet: client.wallet(new MnemonicKey({
    mnemonic: "radar great river eyebrow hat beach lake club melody bench cousin swear remain crack habit tank peanut sort turkey grit envelope segment door desert",
  })),
  myKeyWallet: client.wallet(new MnemonicKey({
    mnemonic: "village skull shell gaze adapt repeat inherit fresh hungry old wall ankle lawn mass leopard seminar carpet access coach above crush worry depart keep",
  })),
  test1Wallet: client.wallet(new MnemonicKey({
    mnemonic: "notice oak worry limit wrap speak medal online prefer cluster roof addict wrist behave treat actual wasp year salad speed social layer crew genius",
  })),
  // terra1qvlw2d4ln60w7mlw6funy7dk6xax3nzesv0pry
  swapOwnerWallet: client.wallet(new MnemonicKey({
    mnemonic: "slide umbrella eagle broken spread world hunt little voyage harvest auction wink wrist large connect lend smooth way clock depend analyst slender need crucial",
  })),
  // terra1stmr2pgs8dn3tnuq3j0c8grn0dy70xhgfup6j6
  bombayWallet: client.wallet(new MnemonicKey({
    mnemonic: "virus economy check sample rural snack suspect peasant purchase text lucky royal edge animal apology romance earth reflect goat quiz ladder mimic inhale young",
  })),
};

export { client, wallets };
