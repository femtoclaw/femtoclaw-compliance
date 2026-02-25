# FemtoClaw Reference Compliance Test Implementation

This repository provides the official compliance test harness for validating FemtoClaw runtime implementations.

## Run all compliance tests

```bash
cargo run --release -- run-all
```

## Run specific domain

```bash
cargo run --release -- run protocol
cargo run --release -- run runtime
cargo run --release -- run capability
cargo run --release -- run security
cargo run --release -- run performance
```

Outputs compliance report in JSON and console format.
