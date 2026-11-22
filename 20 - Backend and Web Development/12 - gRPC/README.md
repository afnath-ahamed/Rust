# gRPC

gRPC defines service operations and messages in Protocol Buffers and supports unary or streaming requests. Specify deadlines and error codes and plan schema evolution.

## Contents

1. [Worked example](#1-worked-example)
2. [Common mistake](#2-common-mistake)

## 1. Worked example

```protobuf
syntax = "proto3";
package notes;
service Notes { rpc Read(ReadRequest) returns (ReadReply); }
message ReadRequest { uint64 id = 1; }
message ReadReply { string title = 1; }
```

## 2. Common mistake

Avoid reusing field numbers for removed fields. Reserve them to protect wire compatibility.

## Practice

Design a request, response, and deadline policy for a list API and compare it to the HTTP version.

## References

- [grpc.io · docs](https://grpc.io/docs/)
- [docs.rs · axum](https://docs.rs/axum/latest/axum/)

---

[← Previous: WebSockets](<../11 - WebSockets/README.md>) | [Back to Backend and Web Development](<../README.md>) | [Next: Caching →](<../13 - Caching/README.md>)
