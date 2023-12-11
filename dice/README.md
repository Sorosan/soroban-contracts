# NFT Emulating Dice on Soroban Preview 11

This repository presents the concept of a on-chain dice that uses Stellars PRNG. The PRNG module described here utilizes a robust CSPRNG (ChaCha20) for pseudo-random number generation within the Stellar network. It is advisable only for applications where the acceptable risk of validator influence exists.

## Getting Started

To initiate this POC, ensure that the Soroban platform is installed and running on your local machine.

Once Soroban is set up, clone this repository and execute the following commands to deploy the NFT contract:

```bash
make            # build the contract (should generate .wasm)
```

This will build the contract, producing the necessary .wasm file for deployment on the Soroban blockchain platform.