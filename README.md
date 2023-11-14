# Backend wesleysoto.dev - Rust

## Overview

This project is a Rust-based backend service designed for efficient web operations with AWS S3 integration. It leverages Actix-web as its web framework and is containerized using Docker, ensuring easy deployment and consistent performance across different environments.

## Features

- **AWS S3 Integration:** Seamlessly interacts with AWS S3 for robust data storage and retrieval.
- **Actix-web Framework:** Employs Actix-web for a high-performance and flexible web server.
- **Docker Support:** Fully containerized, allowing for straightforward setup and scalability.

## Getting Started

### Prerequisites

- Docker installed on your machine.
- Rust 1.73.0 or higher.

### Installation

1. **Clone the Repository:**

   ```sh
   git clone [repository-url]
   cd backend_wesleysoto_dev-rust
   ```

2. **Build the Docker Image:**

   ```sh
   docker build -t backend_wesleysoto_dev .
   ```

### Running the Application

1. **Start the Docker Container:**

   ```sh
   docker run -p 8080:8080 backend_wesleysoto_dev
   ```

2. **Access the application:** The app will be available at `http://localhost:8080`.

## Usage

The application provides various endpoints for web interactions and AWS S3 data management. Specific routes and their functionalities are defined in `routes.rs`. 

## API Reference

- `/navbar-items`: Fetches navigation items from S3.
- Add more routes as applicable.

## Development

For local development:

1. **Environment Setup:** Ensure Rust and the necessary toolchains are installed.
2. **Run Locally:** Use `cargo run` to start the application locally.

## Testing

- Run tests using `cargo test` (if test cases are defined).

## License

All rights reserved.
