# GitHub Rust API Call 🦀

![Rust](https://img.shields.io/badge/Language-Rust-orange)

This Rust project demonstrates how to make API calls to GitHub's REST API using the reqwest crate, retrieve information about stargazers of a repository, and deserialize JSON responses into Rust structs using serde.

## Tech Stack Used

- [Rust 🦀](https://www.rust-lang.org/): The programming language.
- [serde](https://crates.io/crates/serde): A data serialization and deserialization library for Rust.
- [serde_json](https://crates.io/crates/serde_json): JSON support for Serde.
- [tokio](https://crates.io/crates/tokio): An asynchronous runtime for Rust.
- [reqwest](https://crates.io/crates/reqwest): An HTTP client for Rust.

## Project Importance

This project is a practical example of making API calls in Rust, deserializing JSON responses, and handling user input. It's important for developers looking to build applications that interact with web APIs using Rust.

## Setup and Execution

1. Make sure you have Rust and Cargo installed on your system. If not, you can install them from [Rust's official website](https://www.rust-lang.org/learn/get-started).

2. Clone this repository:

```bash
   git clone https://github.com/codescalper/github-rust-api-call.git
   cd github-rust-api-call
```

3.  Build and run the project with Cargo:

```bash
    cargo run
```

The program will prompt you to enter the name of a GitHub user and their repository. It will then retrieve stargazers of the repository and print their data.

## Where It Can Be Used

1.  **GitHub Data Retrieval**: This project can be integrated into applications that require fetching data from GitHub repositories. It's useful for analytics, tracking user engagement, and more.
2.  **API Interaction in Rust**: Developers can use this project as a starting point for making HTTP requests to various APIs and handling the data in their Rust applications.

---

If you have any questions or suggestions, feel free to reach out on Twitter: [@mayanks_tw](https://twitter.com/mayanks_tw).

### Output Screenshot

![Output Screenshot](https://cdn.discordapp.com/attachments/1150040438904979557/1166062190067986562/image.png?ex=65491f0a&is=6536aa0a&hm=f6ca0f1fde2c7fcfa1fd53c28a4ccc1aefcc1979314122ad784f3c76d6365354)
