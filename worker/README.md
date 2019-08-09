# pajero

Packets replayer for attack &amp; defence CTF

## Build

```
git clone git@github.com:GNUp/pajero.git
cd pajero
```

Build in release mode

```
cargo build --release
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

Analyze packet as pre-defined filter

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
