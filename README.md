# pajero

pajero is a packets analyzer for attack & defense CTF which can help you to systematically investigate packets your server received. There is no meaning in pajero. It was just generated by a random project name generator.

pajero is designed to analyze packet dumps that are usually provided from DEFCON organization during the CTF. Especially in DEFCON26, they provided packet dumps for each service we were running intermittently. These include communication between our services and other teams, abnormal usage of services which means attack payload from others, and also the flag of our team that was stolen. By classifying a bunch of packets into team by team and service by service we can easily grasp the overall situation of our services. Also by detecting our flag in the packet we can spot the attack payload which works on our services that also can be used to the same services on other teams' servers. We expect that by replaying this payload we can make somehow valid attacks on other servers. 

## How to use

You can find the most recent release of Mach-O 64-bit executable arm64 binary for the project [here](https://github.com/posgnu/pajero/releases). If you are using other types of OS then you need to clone the source code and need to compile it. For building a binary by yourself, check the [Building section](#Building).

### Set up

Before all,  generate`./static/conf.json`. It should be filled with team information and service configurations following the template below. 

```json
"team" : [
  { "name" : "PLUS", "ip" : "0.0.0.0" } , 
],
"service" : [
  {"name" : "bof", "flag" : "DEFCON{", "port": 8888 }, 
]
```

After populating the team list with other competitors' information, list the services your team is running. The `flag` is an argument for finding an attack payload that leaks our flag so it does not have to be a completed sentence of the flag. Only keywords will be fine. `port` needs, though, some cautions since pajero differentiate packets by port.

### Run

```
$./pajero run analyze [packet path] [round]
```

Analyze packet with pre-defined filter (conf.json) and then we can see the results in the`./static/packets/` directory. Attack payload will be collected in `./static/packets/flag/`.

## Contribution

If you want to make some contributions to this project, here some basic instructions for you.

### Design

![model](https://github.com/GNUp/pajero/blob/master/model.png)

### Target network

![network](https://github.com/GNUp/pajero/blob/master/network.png)

### Building

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

### Testing

```
cargo test -- --test-threads=1
```

Basically, we need to run all the tests synchronously before resolving [#16](https://github.com/GNUp/pajero/issues/26)

## Formatting

Run `rustfmt` before the pushing.

```sh
$ cargo fmt
```

## License

This project is licensed under GNU GPLv3 - see the `LICENSE.txt` for details