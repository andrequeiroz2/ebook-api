# Book API

This repository contains a RESTful API built with Rust using the Actix-web framework for handling HTTP requests and MongoDB for data storage.

## Prerequisites

- Rust ([installation guide](https://www.rust-lang.org/tools/install))

## .env
- Create a .env file at the root of the project, replacing <user>, <pass>, <host> 

- MONGOURI="mongodb+srv://<user>:<pass>@<host>/?retryWrites=true&w=majority"

## Run

```bash
$ cargo run
```

## help

- get: localhost:8080/help