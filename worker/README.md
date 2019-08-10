# pajero

Packets replayer for attack &amp; defence CTF

## Build

First, Install [rustup](https://rustup.rs/) and clone repository.

```shell
$ git clone git@github.com:GNUp/pajero.git
$ cd pajero
```

Install libpcap

```shell
$ sudo apt-get install libpcap-dev
```

Give +x permission to PcapSplitter binary

```shell
$ chmod +x pajero/worker/static/bin/PcapSplitter
```

Use a nightly version of Rust

```shell
$ rustup override set nightly
```

~~Build in release mode~~

```shell
$ cargo build --release
```

This will produce an executable in the `./target/release` directory.

## How to use

```
$./cargo run serve
```

serve APIs for the client

```
$./cargo run analyze [packet path] [round]
```

Analyze packet with pre-defined filter and then we can see the results in the`./static/packets/` directory.

## Testing

```
cargo test -- --test-threads=1
```

Basically, we need to run all the tests synchronously before resolving [#16](https://github.com/GNUp/pajero/issues/26)

## Formatting

```
rustup toolchain install nightly-2019-05-17
rustup component add rustfmt --toolchain nightly-2019-05-17
```

To run `rustfmt`

```
    cargo +nightly-2019-05-17 fmt
```

## License

This project is licensed under GNU GPLv3 - see the `LICENSE.txt` for details
