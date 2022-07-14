## About
This is ordering service (api) , Design base on hexaglonal architecture 

## Features

- Create new order 
- Edit and update order
- Cancel order 
- Submit order

## Stacks

- [Actix] - The Web framework opensource create in Rust 
- [Mongodb] - The database of this service , it's NoSQL
- [Docker] - The container services
- [Redis] - Store logs in memories
- [Kafka] - The message broker events
- [RedLock] - The distributed locking
## Installation

This service create by Rust , If not rust compiler . Then install compiler first.
[Rust](https://www.rust-lang.org/tools/install)

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

And install Mongodb for database service [Mongodb](https://www.mongodb.com/docs/manual/installation/)

Install Redis for logger store [Redis](https://redis.io/docs/getting-started/installation/)

Install Kafka for message queue broker [Kafka](https://www.conduktor.io/kafka/how-to-install-apache-kafka-on-mac)

## Development

Open your projects in favorite terminal and go to directory

Start kafka server

```sh
zookeeper-server-start /usr/local/etc/kafka/zookeeper.properties
```

```sh
kafka-server-start /usr/local/etc/kafka/server.properties
```

Test consume event message queue

```sh
kafka-console-consumer --bootstrap-server localhost:9092 --topic Confirm_Order --from-beginning
```

```sh
cd order_service
```

Download and build dependencies

```sh
cargo build
```

Start service

```sh
cargo run
```

Build for production

```sh
cargo build --release
```

## Testings

Run integrations testing

```sh
cargo test
```

Report tests

![alt text](/src/tests/test_report/Screen Shot 2565-06-18 at 19.31.23.png)

## Docker setup and run


## Issue

When redis cannot add LPUSH , You can check status redis service and

```sh
config set stop-writes-on-bgsave-error no
```

## Author

@Pinyoo Thotaboot







