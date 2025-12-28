# Simple Load balancer with TLS support

## Overview

CP_LB is a load-balancing application designed to handle high-performance HTTP requests with support for upstream health checks. Built with Rust, it uses various crates and modular designs to provide efficient load balancing with advanced features like round-robin peer selection and health check monitoring.

### Core Features:

- **Load Balancing**: Implements round-robin peer selection for upstream servers.
- **Health Checks**: Supports TCP-based health check mechanisms to ensure upstream availability.
- **Proxy HTTP Requests**: Provides configurable HTTP proxy support with upstream filtering.
- **TLS Support**: Secure communication through TLS certificates.
- **Configurable Options**: Allows customization through settings configured within the application.

## Usage
1. Clone the repository:
   ```bash
   git clone <repository-url>
   ```

2. Build the application:
   ```bash
   cargo build --release
   ```

3. Run the application:
   ```bash
   ./target/release/cp_lb
   ```

## Settings

The application reads configurations from the `settings` module and `Settings.toml` file, such as:
- General configurations (e.g., logging levels, server hostnames).
- Upstream addresses for load balancing.
- Interval for health checks.

You can find `Settings.toml.example` in the root directory of the project. It allows for easy customization of the application's behavior and parameters.

## Dependencies

The application uses the following Rust crates:
- `pingora-core`: Core utilities for server and upstreams.
- `pingora-proxy`: Proxy services for HTTP requests.
- `tracing`: Structured logging and debugging information.

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.

## Contribution

Feel free to open issues or submit pull requests for improvements!

