# gRPC server and client in Rust with tonic

A gRPC server and client written in Rust with [tonic](https://github.com/grpc/grpc-rust). The code belongs to the blog post [How to Build a gRPC Server and Client in Rust with tonic](https://thorsten-hans.com/grpc-services-in-rust-with-tonic/).

Tested with Rust 1.100 (edition 2024), tonic 0.14 and prost 0.14.

## Prerequisites

- Rust 1.88 or newer
- `protoc`, the Protocol Buffers compiler (`brew install protobuf` or `apt install protobuf-compiler`). If it is not on your `PATH`, point the `PROTOC` environment variable at the binary.

## Run it

```bash
# terminal 1
cd server
cargo run

# terminal 2
cd client
cargo run
```

The client asks for a URL and an up or down vote, sends it to the server and prints the confirmation. Any other input ends the client.
