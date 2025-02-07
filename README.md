# IMPHNEN CMS API

This project is a CMS API for [IMPHNEN Website](https://imphnen.dev)

## Features

- **RESTful API**: Provides endpoints for managing wedding reservations.
- **Database Integration**: Employs SeaORM for seamless database interactions.
- **Authentication**: Authentication and Middleware
- **CORS Handling**: Handling CORS with Tower HTTP CorsLayer
- **API Docs**: OpenAPI Swager Ready

## Prerequisites

- **Rust**: Ensure that Rust is installed on your system. You can install it from [rust-lang.org](https://www.rust-lang.org/).
- **Database**: Set up a Postgresql database and note the connection details
- **Docker**: if you want build this project using docker, you need docker, You can install it from [docker.com](https://www.docker.com/)
- **Nix**: if you want build this project using nix, you need nix, You can install it from [nixos.org](https://nixos.org/)

## Getting Started

1. **Clone the Repository**:

   ```bash
   git clone https://github.com/IMPHNEN/imphnen-cms-api.git
   cd imphnen-cms-api
   ```

2. **Set Up Environment Variables**:

   Copy a `.env.exanple` file:

   ```env
   cp .env.example .env
   ```

3. **Install Dependencies**:

   ```bash
   cargo check
   ```

   ```bash
   cargo build --release
   ```
    ```bash
   cargo install sea-orm-cli
   ```

5. **Run Database Migrations**:

   ```bash
   sea-orm-cli migrate up
   ```

6. **Start the Server**:

   ```bash
   cargo run -q
   ```

   The API will be accessible at `http://localhost:3000/v1/docs`.

## Docker

1. **Build the Docker Image**:

   ```bash
   docker build -t cms-api .
   ```

2. **Run the Docker Container**:

   ```bash
   docker run -p 3000:3000 --env-file .env cms-api cms-api:latest
   ```

   The API will be accessible at `http://localhost:3000/v1/docs`.


## Using Nix as Builder

1. **Install Nix**:

   ```bash
   curl -L https://nixos.org/nix/install | sh
   ```

2. **Switch to Nix Shell or Nix Flake**:

   ```bash
   nix develop
   ```

3. **Build the Project**:

   ```bash
   nix build
   ```

4. **Run the Server**:

   ```bash
    nix run
   ```


## Contributing

Contributions are welcome! Please fork the repository and create a pull request with your changes.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgements

- [Axum](https://github.com/tokio-rs/axum)
- [SeaORM](https://github.com/SeaQL/sea-orm)
