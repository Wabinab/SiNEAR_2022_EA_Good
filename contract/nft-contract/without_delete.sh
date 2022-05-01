#!/bin/bash

bash build.sh
export CONTRACT=ea_nft.wabinab.testnet

near deploy --accountId $CONTRACT --wasmFile res/output_s.wasm 