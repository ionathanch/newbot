# 🇳​🇪​🇼

[@new@cybre.space](https://cybre.space/@new), but it's [@new@types.pl](https://types.pl/@new).

## Setup

1. Install Rust and generate access token at https://types.pl/settings/applications.
2. Copy access token to `MASTO_TOKEN` in `.cargo/config.toml`.
3. Run `make newbot` to build and `make new` to post.

## Changes

* [types.pl](https://types.pl/) restricts access to the emoji list API,
  so GETing it also requires authorization.
* I don't use AWS Lambda, so those dependencies have been removed.