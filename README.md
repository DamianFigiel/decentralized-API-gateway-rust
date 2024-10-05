# Decentralized API Gateway in Rust

This project is a decentralized API gateway built in Rust. The gateway handles routing, authentication using JWT, WebSocket support, and real-time interaction with the Ethereum blockchain. It can retrieve the latest block from the Ethereum network via Infura.

## Features
- **JWT Authentication**: Protect API routes using JWT tokens.
- **WebSocket Support**: Enable real-time communication with WebSocket connections.
- **Ethereum Integration**: Retrieve the latest block from the Ethereum network.
- **API Gateway**: Acts as a middleware for managing routing and authentication for decentralized applications.

## Project Structure

- `Cargo.toml`: Project configuration and dependencies.
- `src/main.rs`: The main entry point for the Actix Web server.
- `src/blockchain.rs`: Ethereum integration using the `web3` crate to retrieve the latest block.
- `src/jwt.rs`: Handles JWT token creation and validation.
- `src/ws_handler.rs`: WebSocket handler for real-time communication.
- `.env.example`: Example environment configuration.

## Requirements

- **Rust** (1.72 or higher)
- **Infura Project ID** for Ethereum integration.

## Environment Setup

This project uses environment variables to securely store API keys and other sensitive information. You'll need to create a `.env` file in the root of the project.

### 1. Install Rust

Ensure Rust is installed on your system by following the instructions on [rust-lang.org](https://www.rust-lang.org/tools/install).

### 2. Create a `.env` File

You can copy the `.env.example` file to `.env` and fill in your **Infura Project ID**.
    
    cp .env.example .env

Update the .env file with your Infura Project ID:
    
    INFURA_PROJECT_ID=your_infura_project_id_here

You can get your Infura Project ID by signing up on [Infura](https://infura.io/).

### 3. Install Dependencies
Install the required Rust dependencies by running:

    cargo build

## Running the Project
To run the API gateway, simply use the following command:

    cargo run

The server will start, listening on localhost:9090, unless you specify different port in .env file.

## API Endpoints
### 1. JWT Authentication
- Endpoint: /auth
- Method: POST
- Description: Generates a JWT token for authentication.
- Request Body:
    ```json
    {
        "user_id": "your_user_id"
    }
- Response:
    ```json
    {
        "token": "your_jwt_token"
    }
### 2. Secure Endpoint
- Endpoint: /secure
- Method: GET
- Description: Protected route that requires a JWT token in the Authorization header.
- Authorization Header:
    ```makefile
    Authorization: Bearer your_jwt_token
### 3. WebSocket Endpoint
- Endpoint: /ws/
- Description: WebSocket connection for real-time communication.
- Use a WebSocket client like wscat:
    ```bash
    wscat -c ws://localhost:8080/ws/
### 4. Ethereum Latest Block
- Endpoint: /ethereum/latest_block
- Method: GET
- Description: Retrieves the latest block from the Ethereum blockchain.
- Response:
    ```json
    {
        "block_number": 20898871
    }
### Testing
You can use curl or tools like Postman or curl to interact with the API. For example, to get the latest Ethereum block:

    curl http://localhost:8080/ethereum/latest_block

To test JWT-protected routes, first generate a token via /auth, then use the token in subsequent requests:

    curl -X POST http://localhost:8080/auth -H "Content-Type: application/json" -d '{"user_id": "test_user"}'

Use the returned token in the Authorization header to access /secure:

    curl http://localhost:8080/secure -H "Authorization: Bearer your_jwt_token"

## Dependencies
The project uses the following dependencies:
- Actix Web: A powerful, pragmatic, and extremely fast web framework for Rust.
- Web3: Ethereum JSON-RPC client for interacting with Ethereum.
- JWT: Used for generating and validating JSON Web Tokens.
- dotenv: Manage environment variables securely.
- Tokio: Asynchronous runtime for Rust.

## License
This project is licensed under the MIT License.