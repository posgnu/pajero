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
$./pajero set
```
Set the information about teams which are participating the CTF
```
$./pajero analyze
```
Analyze packet as pre-defined filter
```
$./pajero play
```
Replay packet to another team's service
## Formatting
```
rustup toolchain install nightly-2018-07-17
rustup component add rustfmt-preview --toolchain nightly-2018-07-17
```
To run `rustfmt`
```
cargo +nightly-2018-07-17 fmt
```
## License
This project is licensed under GNU GPLv3 - see the `LICENSE.txt` for details
