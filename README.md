# Crypto Library for EigenCC
Modified from [ring](https://github.com/briansmith/ring).

## Test
```
git clone https://github.com/ieigen/eigen-crypto
cd eigen-crypto
export  LANGS=$PWD/hdwallet
cargo test
```

## TODO
* [x] hash/aes/encoder
* [x] address and mnemonic
* [x] ecdsa
* [x] ecies, supported but is not compatible with go-ecies due to different AES used.
* [ ] schnorr and BLS multi-sig
* [ ] bulletproofs
* [ ] HD Wallet(BIP32)
