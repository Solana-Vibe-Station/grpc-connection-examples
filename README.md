# gRPC Connection Examples

A collection of example implementations for connecting to Solana Vibe Station's Yellowstone gRPC service (or other compatible providers) in multiple programming languages. These examples demonstrate common practices for establishing connections, handling reconnections, and processing real-time Solana blockchain data.

## Repository Structure

This repository contains example implementations in the following languages:

```
grpc_connection_examples/
├── rust/          # Rust implementation with Tokio async runtime
├── typescript/    # TypeScript/Node.js implementation
├── python/        # Python implementation with asyncio
└── LICENSE        # MIT License
```

Each language folder contains:
- A complete, working example
- Language-specific README with detailed instructions
- Proper dependency management files
- Common practices for that language ecosystem

## Getting Started

1. **Clone the repository:**
   ```bash
   git clone https://github.com/Solana-Vibe-Station/grpc_connection_examples
   cd grpc_connection_examples
   ```

2. **Choose your language:**
   Navigate to the folder for your preferred programming language:
   ```bash
   cd rust       # For Rust example
   cd typescript # For TypeScript example  
   cd python     # For Python example
   ```

3. **Follow language-specific instructions:**
   Each folder contains its own README with detailed setup and usage instructions.

## Features Demonstrated

All examples showcase:
- ✅ Connecting to Yellowstone gRPC endpoints
- ✅ Authentication with access tokens
- ✅ Subscribing to real-time blockchain data
- ✅ Handling different message types (slots, accounts, transactions, blocks)
- ✅ Automatic reconnection with exponential backoff
- ✅ Proper error handling and logging
- ✅ Clean shutdown procedures

## Connection Information

To use these examples, you'll need:

1. **Endpoint URL**: Your Yellowstone gRPC endpoint
2. **Access Token**: Your authentication token (x-token)

Solana Vibe Station customers can find this information:
- On your Cloud Portal services page
- In our documentation: [docs.solanavibestation.com/introduction/connection-information](https://docs.solanavibestation.com/introduction/connection-information)

**Note**: Discord customers using IP-based authentication can leave the access token empty.

## Language-Specific Documentation

For detailed instructions, configuration options, and customization examples, please refer to the README in each language folder:

- [Rust Documentation](./rust/README.md)
- [TypeScript Documentation](./typescript/README.md)
- [Python Documentation](./python/README.md)

## Requirements

General requirements across all implementations:
- A Yellowstone gRPC endpoint (from Solana Vibe Station or other compatible providers)
- Valid authentication credentials (unless using IP-based auth)
- Language-specific runtime and dependencies (see individual READMEs)

## Contributing

We welcome contributions! If you'd like to:
- Add examples in other languages
- Improve existing examples
- Fix bugs or enhance documentation

Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## About Solana Vibe Station

[Solana Vibe Station](https://solanavibestation.com) provides high-performance infrastructure for the Solana blockchain, including Yellowstone gRPC services for real-time data streaming.

---

For language-specific implementation details, please navigate to the respective folders and consult their README files.