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

By default, otel sends data to `http://localhost:4317` in grpc.

#### Metrics

```sh
otel export metrics gauge \
  --name system.cpu.temerature \
  --description "cpu temperature" \
  --unit Cel \
  --value-as-double 30.0 \
  --attributes key1:val1 \
  --resources thermalzone:0 \
  --schema-url https://opentelemetry.io/schemas/1.21.0
```

## License

This project is licensed under the [MIT license.](./LICENSE)