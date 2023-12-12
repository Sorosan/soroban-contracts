# Dice on Soroban

## Overview

The "Dice on Soroban Preview 11" repository introduces an on-chain dice concept utilizing Stellar's PRNG (Pseudo-Random Number Generator). This PRNG module, powered by the robust CSPRNG (ChaCha20), ensures secure pseudo-random number generation within the Stellar network. It's important to note that this implementation is recommended for applications where the acceptable risk of validator influence exists.

## Key Features

The smart contract, named `DiceContract`, is centered around the `roll` function, allowing users to obtain a random number between 1 and 6. This functionality is facilitated by leveraging Stellar's PRNG to ensure fair and unpredictable outcomes.

## Objectives and Goals of Creating this Smart Contract

The primary objective of this smart contract is to provide a decentralized and trustless mechanism for obtaining random numbers within the Stellar network. By utilizing a robust CSPRNG and Stellar's PRNG capabilities, the contract aims to deliver secure and fair results, making it suitable for applications such as on-chain gaming or any scenario requiring reliable randomness.

In summary, the "Dice on Soroban Preview 11" smart contract presents an innovative approach to on-chain dice rolling, offering a secure and decentralized solution for obtaining random numbers within the Stellar network.