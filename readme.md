# mqtt2statsd

mqtt2statsd is a service that translates numeric MQTT messages into statsd gauge metrics.

## Dependencies

Ubuntu:
```
$ sudo apt install \
libssl-dev \
pkg-config \
cmake \
gcc
```

## Configuration

mqtt2statsd is configured via a toml configuration file.
Check [example.toml](./example.toml) for an example configuration.
