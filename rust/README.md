# Yellowstone gRPC Client

A production-ready Rust client for connecting to Yellowstone gRPC endpoints with automatic reconnection logic. Built by [Solana Vibe Station](https://solanavibestation.com) as a reference implementation for our customers.

> **Note for Solana Vibe Station customers**: This example demonstrates how to connect to our Yellowstone gRPC endpoints. The code is open source and can be used with any compatible Yellowstone gRPC provider.

## Features

- 🔄 **Automatic reconnection** with exponential backoff
- 📊 **Real-time updates** for slots, accounts, transactions, and blocks
- 🏓 **Ping/pong keepalive** support
- 📝 **Structured logging** with tracing
- 🔐 **TLS support** with native roots
- ⚡ **Async/await** with Tokio runtime

## Prerequisites

- Rust 1.70 or higher
- A Yellowstone gRPC endpoint (e.g., from Solana Vibe Station or other providers)
- An access token (if required by your provider)

## Installation

1. Clone the repository:
```bash
git clone https://github.com/Solana-Vibe-Station/grpc_connection_examples
cd grpc_connection_examples/rust
```

2. Create a `.env` file in the rust folder:
```env
# Your Yellowstone gRPC endpoint URL
GEYSER_ENDPOINT=your-endpoint-url

# Your access token (x-token)
# Leave empty if using IP-based authentication
GEYSER_ACCESS_TOKEN=your-x-token
```

## Usage

### Basic Usage

Run the client with default slot subscription:
```bash
cargo run
```

### Example Output
```
[INFO] Connecting to gRPC endpoint: https://yellowstone.solanavibestation.com
[INFO] Successfully connected to Yellowstone gRPC
[INFO] Subscribed to slot updates, waiting for messages...
[INFO] Slot update: slot=276389641, parent=276389640, status=Confirmed
[INFO] Received ping from server - replying to keep connection alive
[INFO] Slot update: slot=276389642, parent=276389641, status=Confirmed
```

## Configuration

The client reads configuration from environment variables:

| Variable | Description | Required |
|----------|-------------|----------|
| `GEYSER_ENDPOINT` | The gRPC endpoint URL | Yes |
| `GEYSER_ACCESS_TOKEN` | Authentication token | No (depends on provider) |

### Solana Vibe Station Configuration

Solana Vibe Station customers can find their connection information:
- **Endpoint URL**: Available on your Cloud Portal services page or at [docs.solanavibestation.com/introduction/connection-information](https://docs.solanavibestation.com/introduction/connection-information)
- **Access Token (x-token)**: Found on your Cloud Portal services page
- **Discord customers**: Leave `GEYSER_ACCESS_TOKEN` empty as authentication is IP-based

## Reconnection Logic

The client implements robust reconnection logic:

1. **Initial Connection**: Retries with exponential backoff if connection fails
2. **Stream Interruption**: Automatically reconnects when the stream ends or errors occur
3. **Graceful Handling**: Handles both error disconnections and normal stream termination

## Customization

### Subscribing to Different Data

Modify the `SubscribeRequest` in `run_subscription()` to subscribe to different data types:

```rust
// Example: Subscribe to specific accounts
let request = SubscribeRequest {
    accounts: std::collections::HashMap::from([
        ("my_accounts".to_string(), SubscribeRequestFilterAccounts {
            account: vec!["11111111111111111111111111111111".to_string()],
            ..Default::default()
        })
    ]),
    commitment: Some(CommitmentLevel::Confirmed as i32),
    ..Default::default()
};
```

### Available Subscription Types

- **Slots**: Block production updates
- **Accounts**: Account state changes
- **Transactions**: Transaction notifications
- **Blocks**: Full block data
- **Block Meta**: Block metadata only

## Dependencies

- `yellowstone-grpc-client` - Yellowstone gRPC client library
- `tokio` - Async runtime
- `tonic` - gRPC framework
- `backoff` - Exponential backoff for retries
- `tracing` - Structured logging
- `anyhow` - Error handling

## Troubleshooting

### Connection Issues
- Verify your endpoint URL is correct
- Check if your access token is valid
- Ensure your IP is whitelisted (if required)

### Stream Closes Immediately
- Check subscription filters - invalid filters may cause immediate disconnection
- Verify commitment level is supported by your provider

### High Memory Usage
- Consider filtering subscriptions more specifically
- Implement data processing in separate tasks

## License

This project is licensed under the MIT License - see the [LICENSE](../LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.