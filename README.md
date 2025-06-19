# Solana Pinocchio Examples

This repository contains examples demonstrating how to use Pinocchio, a zero-dependency library for creating Solana programs in Rust.

## What is Pinocchio?

Pinocchio is a lightweight library that allows you to write Solana programs without depending on the `solana-program` crate. It leverages zero-copy types to read program input parameters directly from the byte array passed to the program's entrypoint by the SVM (Solana Virtual Machine) loader.

### Key Benefits:

- **Zero dependencies**: Eliminates the need for `solana-program` crate
- **Resolves dependency conflicts**: Avoids versioning issues with the Solana runtime
- **Lightweight**: Minimizes program size and complexity
- **Efficient**: Uses zero-copy deserialization for better performance

## The example programs

<details>
  <summary>Basics</summary>

### Hello world

[Hello World on Solana! A minimal program that logs a greeting.](./basics/hello-solana)

### Account-data

[Store and retrieve data using Solana accounts.](./basics/account-data)

### Storing global state - Counter

[Use a PDA to store global state, making a counter that increments when called.](./basics/counter)

### Saving per-user state - Favorites

[Save and update per-user state on the blockchain, ensuring users can only update their own information.](./basics/favorites)

### Checking Instruction Accounts

[Check that the accounts provided in incoming instructions meet particular criteria.](./basics/checking-accounts)

### Closing Accounts

[Close an account and get the Lamports back.](./basics/close-account)

### Creating Accounts

[Make new accounts on the blockchain.](./basics/create-account)

### Cross program invocations

[Invoke an instruction handler from one onchain program in another onchain program.](./basics/cross-program-invocation)

### PDA rent-payer

[Use a PDA to pay the rent for the creation of a new account.](./basics/pda-rent-payer)

### Processing instructions

[Add parameters to an instruction handler and use them.](./basics/processing-instructions)

### Storing date in program derived addresses

[Store and retrieve state in Solana.](./basics/program-derived-addresses)

### Handling accounts that expand in size

[How to store state that changes size in Solana.](./basics/realloc)

### Calculating account size to determine rent

[Determine the necessary minimum rent by calculating an account's size.](./basics/rent)

### Laying out larger programs

[Layout larger Solana onchain programs.](./basics/repository-layout)

### Transferring SOL

[Send SOL between two accounts.](./basics/transfer-sol)

</details>
<details>
  <summary>Tokens</summary>

### Creating tokens

[Create a token on Solana with a token symbol and icon.](./tokens/create-token)

### Minting NFTS

[Mint an NFT from inside your own onchain program using the Token and Metaplex Token Metadata programs.](./tokens/nft-minter) Reminder: you don't need your own program just to mint an NFT, see the note at the top of this README.

### Minting a token from inside a program

[Mint a Token from inside your own onchain program using the Token program.](./tokens/spl-token-minter) Reminder: you don't need your own program just to mint an NFT, see the note at the top of this README.

### Transferring Tokens

[Transfer tokens between accounts](./tokens/transfer-tokens)

### Allowing users to swap digital assets - Escrow

[Allow two users to swap digital assets with each other, each getting 100% of what the other has offered due to the power of decentralization!](./tokens/escrow)

### Minting a token from inside a program with a PDA as the mint authority

[Mint a Token from inside your own onchain program using the Token program.](./tokens/pda-mint-authority) Reminder: you don't need your own program just to mint an NFT, see the note at the top of this README.

### Creating an Automated Market Maker

[Create liquidity pools to allow trading of new digital assets and allows users that provide liquidity to be rewarded by creating an Automated Market Maker.](./tokens/token-swap)

</details>

<details>

  <summary>Token Extensions</summary>

### Basics - create token mints, mint tokens, and transfer tokens with Token Extensions

[Create token mints, mint tokens, and transferr tokens using Token Extensions.](./tokens/token-2022/basics)

### Preventing CPIs with CPI guard

[Enable CPI guard to prevents certain token action from occurring within CPI (Cross-Program Invocation).](./tokens/token-2022/cpi-guard)

### Using default account state

[Create new token accounts that are frozen by default.](./tokens/token-2022/default-account-state)

### Grouping tokens

[Create tokens that belong to larger groups of tokens using the Group Pointer extension.](./tokens/token-2022/group)

### Creating token accounts whose owner cannot be changed

[Create tokens whose owning program cannot be changed.](./tokens/token-2022/immutable-owner)

### Interest bearing tokens

[Create tokens that show an 'interest' calculation.](./tokens/token-2022/interest-bearing)

### Requiring transactions to include descriptive memos

[Create tokens where transfers must have a memo describing the transaction attached.](./tokens/token-2022/memo-transfer)

### Adding on-chain metadata to the token mint

[Create tokens that store their onchain metadata inside the token mint, without needing to use or pay for additional programs.](./tokens/token-2022/metadata)

### Allow a designedated account to close a mint

[Allow a designated account to close a Mint.](./tokens/token-2022/mint-close-authority)

### Usng multiple token extensions

[Use multiple Token Extensions at once.](./tokens/token-2022/multiple-extensions)

### Non-transferrable - create tokens that can't be transferred.

[Create tokens that cannot be transferred.](./tokens/token-2022/non-transferable)

### Permanent Delegate - Create tokens permanently under the control of a particular account

[Create tokens that remain under the control of an account, even when transferred elsewhere.](./tokens/token-2022/permanent-delegate)

### Create tokens with a transfer-fee.

[Create tokens with an inbuilt transfer fee.](./tokens/token-2022/transfer-fee)

</details>
<details>

<summary>Compression</summary>

### Cnft-burn

[Burn compressed NFTs.](./compression/cnft-burn)

### Cnft-vault

[Store Metaplex compressed NFTs inside a PDA.](./compression/cnft-vault)

### Cutils

[Work with Metaplex compressed NFTs.](./compression/cutils)

</details>

<details>

<summary>Oracles</summary>

### pyth

[Use a data source for offchain data (called an Oracle) to perform activities onchain.](./oracles/pyth)

</details>

## Getting Started

### Prerequisites

- Rust and Cargo
- Solana CLI tools
- Basic knowledge of Solana program development

### Building and Testing

```bash
# Clone the repository
git clone https://github.com/yourusername/solana-pinocchio-examples.git
cd solana-pinocchio-examples
bun i


# Build the program
cargo build-sbf
# or
cargo-build-sbf

# Run tests
cargo test
# or
cargo test -p example_program


# Test on devnet

# Get program ID
solana address -k target/deploy/example_program-keypair.json

# Deploy
solana program deploy target/deploy/example_program.so --program-id ./target/deploy/example_program-keypair.json

# Make sure you have update program ID in the codama node struct for generate client code
bun gen:client:example_program
bun test:client:example_program

```

## How to create a new Pinocchio program

To create a new Pinocchio program you need install Rust and the Solana CLI tools. Then you can use the following steps:

1. Create a new Rust library project:

   ```bash
   cargo new --lib my_pinocchio_program --edition 2021
   ```

   cd my_pinocchio_program

   ```

   ```

2. Install the Pinocchio crate:
   ```bash
   cargo add pinocchio
   ```

That is all you need to do to set up a new Pinocchio program. Let start coding!

## Pinocchio Program directory structure

<!-- cấu trúc thư mục lấy cảm hứng từ anchor multiple template -->

The typical directory structure for a Pinocchio program inspired by the Anchor framework looks like this:

```
my_pinocchio_program/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── instructions/
|   |   └── mod.rs
│   ├── state/
|   |   └── mod.rs
│   ├── error.rs
|   ├── constants.rs
|   └── util.rs
├── tests/
│   └── integration_tests.rs
└── target/
```

## Resources

- [Pinocchio GitHub Repository](https://github.com/anza-xyz/pinocchio)
- [Pinocchio Documentation](https://docs.rs/pinocchio)
- [Solana Programming Model](https://docs.solana.com/developing/programming-model/overview)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
