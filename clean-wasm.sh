#!/bin/sh

# cleans all wasm targets

erdpy --verbose contract clean "contracts/benchmarks/str-repeat"
erdpy --verbose contract clean "contracts/examples/adder"
erdpy --verbose contract clean "contracts/examples/crowdfunding-moa"
erdpy --verbose contract clean "contracts/examples/crowdfunding-erc20"
erdpy --verbose contract clean "contracts/examples/crowdfunding-dct"
erdpy --verbose contract clean "contracts/examples/crypto-bubbles"
erdpy --verbose contract clean "contracts/examples/factorial"
erdpy --verbose contract clean "contracts/examples/lottery-moa"
erdpy --verbose contract clean "contracts/examples/lottery-erc20"
erdpy --verbose contract clean "contracts/examples/multisig"
erdpy --verbose contract clean "contracts/examples/simple-erc20"
erdpy --verbose contract clean "contracts/feature-tests/basic-features"
erdpy --verbose contract clean "contracts/feature-tests/async/async-alice"
erdpy --verbose contract clean "contracts/feature-tests/async/async-bob"
erdpy --verbose contract clean "contracts/feature-tests/use-module"
