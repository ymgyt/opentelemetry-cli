# opentelemetry-cli

opentelemetry-cli(`otel`) is a utility cli that provides functionality related to opentelemetry.

## Install

### Cargo 

```sh
cargo install opentelemetry-cli
```

### Nix

```sh
nix run github:ymgyt/opentelemetry-cli -- --help
```

## Features

### Export telemetry data with otlp

#### Metrics

```sh
otel export metrics --todo
```

## License

This project is licensed under the [MIT license.](./LICENSE)