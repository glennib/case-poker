# case-poker

Solution for a case exercise involving poker hands

## run

The server runs in a Docker container, managed by Docker Compose.
To start the server:

```shell
docker compose up
```

The server should then run on your local TCP-port 8080.

## endpoints

* `/draw`
    * `GET` generates a hand of five cards, returns a JSON representation of it and a classification of the hand.
* `/analyze/:cards`
    * `GET` analyzes the provided cards. The `:cards` format is a comma-separated list of rank and suit for five cards.
      Example: `/analyze/tr,jr,qr,kr,1r` would return the JSON string "StraightFlush".

## code documentation

If you have [Rust](https://rustup.rs/) on your system, you can generate and open documentation for the server:

```shell
cd server
cargo doc --no-deps --open
```
