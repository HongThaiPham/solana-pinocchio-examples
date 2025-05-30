# Solana Pinocchio Examples

This repository contains examples demonstrating how to use Pinocchio, a zero-dependency library for creating Solana programs in Rust.

## What is Pinocchio?

Pinocchio is a lightweight library that allows you to write Solana programs without depending on the `solana-program` crate. It leverages zero-copy types to read program input parameters directly from the byte array passed to the program's entrypoint by the SVM (Solana Virtual Machine) loader.

### Key Benefits:

- **Zero dependencies**: Eliminates the need for `solana-program` crate
- **Resolves dependency conflicts**: Avoids versioning issues with the Solana runtime
- **Lightweight**: Minimizes program size and complexity
- **Efficient**: Uses zero-copy deserialization for better performance

## Examples

This repository contains sample programs that demonstrate:

1. Basic Pinocchio program structure
2. Reading account data with zero-copy types
3. Handling program inputs and instructions
4. Cross-program invocation patterns
5. Error handling

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

# go to example directory
cd example_program

# Build the program
cargo build-sbf

# Run tests
cargo test

# Get program ID
solana address -k target/deploy/example_program-keypair.json

# Deploy
solana program deploy target/deploy/example_program.so --program-id ./target/deploy/example_program-keypair.json
```

## How It Works

Pinocchio takes advantage of how the SVM loaders serialize program input parameters into a byte array that is passed to the program's entrypoint. By defining custom zero-copy types that can read directly from this byte array, Pinocchio eliminates the need for the `solana-program` crate.

This approach allows for:

- Direct reading of account data
- Custom instruction data parsing
- Creating programs that work with the Solana runtime without tight coupling

## Resources

- [Pinocchio GitHub Repository](https://github.com/anza-xyz/pinocchio)
- [Pinocchio Documentation](https://docs.rs/pinocchio)
- [Solana Programming Model](https://docs.solana.com/developing/programming-model/overview)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
