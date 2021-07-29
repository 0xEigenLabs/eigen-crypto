# Crypto Library for EigenCC
Universal crypto library for Eigen run both in SGX and Non-SGX.

## Requirement
Rust Version: rustup default nightly-2020-10-25

Rust SGX SDK: 1.1.3

## Test
NOTE: This need some trick right now. you have to un-comment the deps and features in Cargo.toml.

```
git clone https://github.com/ieigen/eigen-crypto
cd eigen-crypto
# Non-SGX
cargo test -- --test-threads 1

# SGX
cargo build --features=mesalock_sgx
cd sgx-test
make run
```

## TODO
* [x] hash/aes/encoder
* [x] address and mnemonic
* [x] ecdsa
* [x] ecies
* [ ] schnorr and BLS multi-sig
* [ ] bulletproofs
* [x] HD Wallet(BIP32)
