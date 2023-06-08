# Demo example for pallet-open-rollup

A demo off-chain miden program of computing Fibonacci sequence.

## Overview

One zkapp execute transactions outside of Blockchain, but periodically commit off-chain transaction batches to an on-chain pallet-open-rollup. 

The on-chain pallet-open-rollup tracks deposits, monitors state updates and verifies zero-knowledge proofs submitted by zkapps.

## Use polkadot.js to interact with pallet-open-rollup

1. Run a development substrate node with pallet-open-rollup
```
git clone https://github.com/open-rollup/open-rollup-node
cd open-rollup-node
git submodule update --init
cargo run --release -- --dev
```

2. Open https://polkadot.js.org/apps/?rpc=ws%253A%252F%252F127.0.0.1%253A9944 to connect the development node

![polkadot-js](/polkadot-js.png)

3. Run the program to genetate program_hash, zk-proof, and zk-outputs files

```
cargo run --release
```

The command compiles and runs the `Miden.masm` program, generating the `miden_program_hash.txt`, `miden.proof`, and `miden.outputs` files.

4. register the program into pallet-open-rollup

![register](/register.png)

5. interact with pallet-open-rollup

You can deposit `Currency` into the registered program, set the program to be inactive, and exit to withdraw `Currency`.

6. Submit the proof file to prove that the program works correctly

Select the generated proof and output files to submit. Miden.masm only computes the Fibonacci sequence, does not maintain the user's assets Merkle tree, and does not execute user transactions. Therefore, the `new_state_root` may be a random value. `l1OperationsPos` should be kept at 0.

![submit](/submit.png)

