# Crypto Library for EigenCC
Modified from [ring](https://github.com/briansmith/ring).

## Test
```
git clone https://github.com/ieigen/eigen-crypto
cd eigen-crypto/sgx-test
cargo build
./bin/run-tests
```

## TODO
* [x] hash/aes/encoder
* [x] address and mnemonic
* [x] ecdsa
* [x] ecies, supported but is not compatible with go-ecies due to different AES used.
* [ ] schnorr and BLS multi-sig
* [ ] bulletproofs
* [x] HD Wallet(BIP32)
