# Basic Peer-to-Peer network using the libp2p concepts and protocols

## Libp2p protocols

The P2P Network uses the following Libp2p protocols:

### Transport Layer

- Noise Protocol: Authentication and end-to-end encryption
- Yamus: stream multiplexing

### Network Behaviour

- Multicast DNS: Peer Discovery within a local network
- Kademlia: Distributed Hash Table (DHT) for peer routing and publishing key-value records
- GossipSub: Publishing messages to specific topic in the network and subscribing to these topic

## USAGE

Start two peers in different terminal windows:

```sh
$ cargo run

p2p 0.1.0
Elena Frank
CLI for the p2p-network interaction

USAGE:
    p2p [SUBCOMMAND]

SUBCOMMANDS:
    connect        explicitly connect a new peer
    get-record     query for a kademlia record
    publish        publish data to certain gossip-sub topic
    put-record     publish a record to the kademlia DHT
    shutdown       shutdown the app
    subscribe      subscribe to a gossip-sub topic
    unsubscribe    unsubscribe from a gossip-sub topic

Started Listening on: /ip4/127.0.0.1/tcp/35361
Local peer Id: PeerId("12D3KooWBJTyq5sNop5PSkw5Yvc56t3HGgVQ27SYrwUvYLY42EW4")
```

Run different commands, e.g.:

```sh
p2p subscribe -t my_topic
```

Publishing a message to a topic requires that at least one other peer exists that is subscribing to that topic.

***Note:** The command line interface only parses input that contains the `p2p` keyword.*

### Explicitly connecting a peer

Most networks support multicast DNS, hence peers within the same network should automatically be discovered.
In case that peers fail to communicate (which causes that e.g. one peer published a record to its DHT, but the other peer can not query it), a peer can explicitely be connect by address:

**Target Peer:**

```sh
... 
Started Listening on: /ip4/127.0.0.1/tcp/32997
Local peer Id: PeerId("12D3KooWQGKtjQbm1A8L7omSywQSnHzLRVRWUyfKS5rDgGoyVA7f")
```

**Source Peer:**

```sh
$ p2p connect -a /ip4/127.0.0.1/tcp/32997
> Successfully connected to Peer 12D3KooWQGKtjQbm1A8L7omSywQSnHzLRVRWUyfKS5rDgGoyVA7f.
```

### Cross Compiling

The programm can be cross-compiled for 64-bit RISC-V with Linux kernel:

```sh
$ rustup target add riscv64gc-unknown-linux-gnu
$ cargo build --target riscv64gc-unknown-linux-gnu
```
