# P2P Server

## Build and run

* Run first peer: `cargo run -- --period=<time_in_seconds> --port=<port_number>`

* Run peer and connect to another: `cargo run -- --period=<time_in_seconds> --port=<port_number> --connect=<address>`

## Example

```
cargo run -- --period=5 --port=1234 > /dev/zero &
cargo run -- --period=6 --port=1235 --connect="127.0.0.1:1234" > /dev/zero &
cargo run -- --period=7 --port=1236 --connect="127.0.0.1:1234"
```

## Test

```
chmod +x tests_bash/test.sh
./tests_bash/test.sh
```