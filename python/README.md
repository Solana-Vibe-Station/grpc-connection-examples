# Python Yellowstone gRPC Client

A production-ready Python client for connecting to Yellowstone gRPC endpoints with automatic reconnection logic. Built by [Solana Vibe Station](https://solanavibestation.com) as a reference implementation for our customers.

> **Note for Solana Vibe Station customers**: This example demonstrates how to connect to our Yellowstone gRPC endpoints. The code is open source and can be used with any compatible Yellowstone gRPC provider.

## Features

- 🔄 **Automatic reconnection** with exponential backoff
- 📊 **Real-time updates** for slots, accounts, transactions, and blocks
- 🏓 **Ping/pong keepalive** support
- 📝 **Structured logging** with Python's logging module
- 🔐 **TLS support** with SSL credentials
- ⚡ **Async/await** with asyncio for high performance
- 🛑 **Graceful shutdown** with clean Ctrl+C handling

## Prerequisites

- Python 3.8 or higher
- A Yellowstone gRPC endpoint (e.g., from Solana Vibe Station or other providers)
- An access token (if required by your provider)

## Installation

1. Clone the repository:
```bash
git clone https://github.com/Solana-Vibe-Station/grpc_connection_examples
cd grpc_connection_examples/python
```

2. Create a virtual environment (recommended):
```bash
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate
```

3. Install dependencies:
```bash
pip install -r requirements.txt
```

4. Create a `.env` file from the example:
```bash
cp .env.example .env
```

5. Edit `.env` with your connection details:
```env
# Your Yellowstone gRPC endpoint
GEYSER_ENDPOINT=your-endpoint-here

# Your access token (x-token)
GEYSER_ACCESS_TOKEN=your-token-here
```

## Configuration

The client reads configuration from environment variables:

| Variable | Description | Required |
|----------|-------------|----------|
| `GEYSER_ENDPOINT` | The gRPC endpoint URL with port | Yes |
| `GEYSER_ACCESS_TOKEN` | Authentication token (x-token) | No (depends on provider) |

### Solana Vibe Station Configuration

Solana Vibe Station customers can find their connection information:

#### Discord Customers:
- **Endpoint**: `grpc.solanavibestation.com:443`
- **Access Token**: Leave empty (authentication is IP-based)

#### Cloud Platform Customers:
- **Endpoints** (based on your tier):
  - Basic: `basic.grpc.solanavibestation.com:443`
  - Ultra: `ultra.grpc.solanavibestation.com:443`
  - Elite: `elite.grpc.solanavibestation.com:443`
- **Access Token (x-token)**: Found in your Cloud Portal service settings

For more details, visit: [docs.solanavibestation.com/introduction/connection-information](https://docs.solanavibestation.com/introduction/connection-information)

## Usage

### Basic Usage

Run the client with default slot subscription:
```bash
python main.py
```

### Example Output
```
2025-08-15 16:00:44 - INFO - Configuration loaded - Endpoint: basic.grpc.solanavibestation.com:443
2025-08-15 16:00:44 - INFO - Connecting to gRPC endpoint: basic.grpc.solanavibestation.com:443
2025-08-15 16:00:44 - INFO - Successfully connected to Yellowstone gRPC
2025-08-15 16:00:44 - INFO - Subscribed to slot updates, waiting for messages...
2025-08-15 16:00:44 - INFO - Slot update: slot=360296291, parent=360296290, status=Confirmed
2025-08-15 16:00:48 - INFO - Received ping from server (id=1) - replying to keep connection alive
2025-08-15 16:00:48 - INFO - Received pong response with id: 1
```

### Graceful Shutdown

Press `Ctrl+C` to shut down gracefully:
```
^C2025-08-15 16:00:50 - INFO - Shutting down gracefully...
2025-08-15 16:00:50 - INFO - Stream closed
```

## Reconnection Logic

The client implements robust reconnection logic:

1. **Initial Connection**: Retries with exponential backoff if connection fails
2. **Stream Interruption**: Automatically reconnects when the stream ends or errors occur
3. **Graceful Handling**: Handles both error disconnections and normal stream termination
4. **Infinite Retry**: Continues attempting to reconnect indefinitely with backoff

## Customization

### Subscribing to Different Data

Modify the `SubscribeRequest` in the `SubscriptionManager.run()` method to subscribe to different data types:

```python
# Example: Subscribe to specific accounts
request = SubscribeRequest()
request.commitment = CommitmentLevel.CONFIRMED

# Add account filter
account_filter = SubscribeRequestFilterAccounts()
account_filter.account.extend([
    "11111111111111111111111111111111",
    "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
])
request.accounts["my_accounts"].CopyFrom(account_filter)

# Add transaction filter
tx_filter = SubscribeRequestFilterTransactions()
tx_filter.account_include.extend(["YourProgramIDHere"])
tx_filter.failed = False
request.transactions["my_txs"].CopyFrom(tx_filter)
```

### Available Subscription Types

- **Slots**: Block production updates
- **Accounts**: Account state changes
- **Transactions**: Transaction notifications
- **Blocks**: Full block data
- **Block Meta**: Block metadata only

## Project Structure

```
python/
├── main.py                 # Main client implementation
├── requirements.txt        # Python dependencies
├── .env.example           # Example configuration
├── protos/                # Protocol buffer definitions
│   ├── geyser.proto
│   └── solana-storage.proto
└── generated/             # Generated protobuf files
    ├── geyser_pb2.py
    ├── geyser_pb2_grpc.py
    ├── solana_storage_pb2.py
    └── solana_storage_pb2_grpc.py
```

## Dependencies

- `grpcio` - gRPC Python implementation
- `grpcio-tools` - Protocol buffer compiler and gRPC tools
- `python-dotenv` - Environment variable management
- `backoff` - Exponential backoff for retries

## Troubleshooting

### Connection Issues
- Verify your endpoint URL includes the port (e.g., `:443`)
- Check if your access token is correct
- Ensure your IP is whitelisted (for Discord customers)

### Installation Issues
If you encounter errors installing `grpcio`:
```bash
# Update pip and install build tools
python -m pip install --upgrade pip setuptools wheel

# Try installing with pre-built wheels
pip install grpcio --no-cache-dir
```

### Stream Closes Immediately
- Check subscription filters - invalid filters may cause immediate disconnection
- Verify commitment level is supported
- Ensure proper authentication

### Import Errors
If you get import errors for generated files:
- Ensure you're running from the `python/` directory
- Check that the `generated/` directory exists with all `.py` files

## Advanced Usage

### Custom Message Handler
Extend the `MessageHandler` class to implement custom logic:

```python
class CustomMessageHandler(MessageHandler):
    async def handle_message(self, update, stub) -> bool:
        # Your custom logic here
        return await super().handle_message(update, stub)
```

### Running with Different Log Levels
```bash
# Set log level via environment variable
LOG_LEVEL=DEBUG python main.py
```

## License

This project is licensed under the MIT License - see the [LICENSE](../LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.