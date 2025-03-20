
# Introduction

This is a quickstart example of how to use the ETOPay SDK to create a new wallet and generate the first receiver address.

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

