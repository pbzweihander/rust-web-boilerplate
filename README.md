# rust-web-boilerplate

[![CircleCI Badge]][CircleCI]
[![MIT License Badge]][MIT License]

Are we web yet? Yes, almost! A boilerplate for working web server with Rust.

## Setup

This boilerplate uses [tide], which requires [futures 0.3], which requires the nightly version of rust.
You can setup nightly rust just for this project with following command:

```bash
rustup override set nightly-2019-05-11
```

## Usage

Cool help messages from [structopt].

```bash
$ cargo run -- --help
rust-web-boilerplate 0.1.0
pbzweihander <pbzweihander@gmail.com>

USAGE:
    rust-web-boilerplate [OPTIONS]

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -h, --host <host>     [default: 0.0.0.0]
    -p, --port <port>     [default: 5000]

$ cargo run -- -h localhost -p 8080
Server is listening on: http://[::1]:8080

# On another terminal...
$ curl localhost:8080/ping
OK

$ curl localhost:8080/hello/rustacean
Hello, rustacean!

$ curl -d "I love Rust" localhost:8080/hello/ferris
Hello, ferris!
Your message was "I love Rust".
```

## Test

With [tide] and [http-service-mock], you can write some integration tests for your web app! Check out [the test code][integration test code].

```bash
cargo test
```

---

_rust-web-boilerplate_ is distributed under the terms of the [MIT License]


[CircleCI Badge]: https://circleci.com/gh/pbzweihander/rust-web-boilerplate.svg?style=svg
[CircleCI]: https://circleci.com/gh/pbzweihander/rust-web-boilerplate
[MIT License Badge]: https://badgen.net/badge/license/MIT/blue
[MIT License]: LICENSE
[tide]: https://github.com/rustasync/tide
[futures 0.3]: https://github.com/rust-lang-nursery/futures-rs
[structopt]: https://github.com/TeXitoi/structopt
[http-service-mock]: https://github.com/rustasync/http-service
[integration test code]: tests/integration.rs
