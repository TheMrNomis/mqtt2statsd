# mqtt2statsd

mqtt2statsd is a service that translates numeric MQTT messages into statsd gauge metrics.

Licensed under the [MIT License](LICENSE.txt).

The main repository for the code is [git.homnomnom.fr/nomis/mqtt2statsd](https://git.homnomnom.fr/nomis/mqtt2statsd), but a duplicate is available on [github/TheMrNomis/mqtt2statsd](https://github.com/TheMrNomis/mqtt2statsd).
Both repositories are kept in sync, and contributions are welcome on both.

## Installing and running

Currently, only supported on debian-based distributions.
For other distributions, see the "Building" section below.

Download the provided `.deb` file, and install it using `apt`:
```bash
$ sudo apt install ./mqtt2statsd_0.1.0_amd64.deb
```

Configure the MQTT and StasD server, as well as the topics to subscribe to in the
configuration file `/etc/mqtt2statsd/config.toml`.

Once configured, enable and start the systemd service:
```bash
$ sudo systemctl enable mqtt2statsd.service
$ sudo systemctl start mqtt2statsd.service
```

## Configuration

mqtt2statsd is configured via a toml configuration file.
Check [example.toml](./example.toml) for an example configuration.

## Building

To build the package, you need the following dependencies (shown here for Ubuntu, actual package names may vary slightly)
```bash
$ sudo apt install \
libssl-dev \
pkg-config \
cmake \
gcc
```

Then, install the latest version of rust with `rustup` ([documentation here](https://rustup.rs/)).

If building the debian package, install `cargo-deb` with:
```bash
$ cargo install cargo-deb
```

Clone the repository, and build the package:
```bash
$ git clone https://git.homnomnom.fr/nomis/mqtt2statsd.git
$ cd mqtt2statsd
$ cargo build --release
$ cargo deb # optional, if building deb package
```

You can now run the mqtt2statsd:
```bash
$ ./target/release/mqtt2statsd ./example.toml
```
