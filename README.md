# ETOPay SDK Rust Quickstart

[![ci](https://github.com/ETOSPHERES-Labs/etopay-sdk-quickstart-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/ETOSPHERES-Labs/etopay-sdk-quickstart-rs/actions/workflows/ci.yml)


This is a quickstart example of how to use the [ETOPay SDK](https://github.com/ETOSPHERES-Labs/etopay-sdk) to create a new wallet and generate the first receiver address.

## Adding the dependency

> The Rust SDK is currently in a pre-release state and is not yet available on the crates.io registry. 

You can access the SDK by adding the following dependency to your `Cargo.toml` file:

```toml
[dependencies]
etopay-sdk = { git = "https://github.com/ETOSPHERES-Labs/etopay-sdk", branch = "main"}
```
or

```toml
[dependencies]
etopay-sdk = "0.14.0-rc0"
```

You can also add the dependency using

```shell
cargo add etopay-sdk
```

## Getting started

- Copy the .env.example file to .env and set the missing values
- Go to https://etopayapp.etospheres.com and get the SDK configuration for you project
- Set the SDK configuration in main.rs
- Run the example with `cargo run`

## Snippets

Curl snippet to get an access_token.

```bash
curl -X POST "https://auth-etopay.etospheres.com/realms/<realm>/protocol/openid-connect/token" \
     -H "Content-Type: application/x-www-form-urlencoded" \
     -d "grant_type=password" \
     -d "scope=profile email openid" \
     -d "client_id=<client_id>" \
     -d "client_secret=<client_secret>" \
     -d "username=<user_name>" \
     -d "password=<user_password>"
```

