# pajero

Packets replayer for attack &amp; defence CTF

## Build

Install rust via [rustup](https://rustup.rs/) and clone the repository.

```shell
$ git clone git@github.com:GNUp/pajero.git
$ cd pajero
```

Install libpcap

```shell
$ sudo apt-get install libpcap-dev
```

Build in release mode

```shell
$ cargo build --release
```

This will produce an executable in the `./target/release` directory.

## Setup

Before all, you should fill `./static/conf.json`. Below is a template for it.

```json
"team" : [
  { "name" : "PLUS", "ip" : "0.0.0.0" } , 
],
"service" : [
  {"name" : "bof", "flag" : "DEFCON{", "port": 8888 }, 
]
```

## How to use

```
$./cargo run serve
```

serve APIs for the client

```
$./cargo run analyze [packet path] [round]
```

Analyze packet with pre-defined filter and then we can see the results in the`./static/packets/` directory.

## Model
![model](https://github.com/GNUp/pajero/blob/master/model.png)
## Target network
![network](https://github.com/GNUp/pajero/blob/master/network.png)

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