![Marlin Oyster Logo](./logo.svg)

# TCP Proxies

This repository contains TCP proxies used to bridge between IP interfaces and vsock interfaces. They are primarily used in the salmon family of images. This repository contains the following proxies:
- ip-to-vsock
- vsock-to-ip
- ip-to-vsock-transparent
- vsock-to-ip-transparent
- port-to-vsock-transparent

## Build

```bash
cargo build --release
```

### Reproducible builds

Reproducible builds can be done using a Rust Docker image to standardize the build environment:

```bash
# For amd64
docker run --rm -v `pwd`:/code rust@sha256:ed7795c6eaccae53be35939e883e8c3de0197b21e8eddbd9f04b0c4bc757c094 /code/build-amd64.sh

# For arm64
docker run --rm -v `pwd`:/code rust@sha256:c428882ff081342a9661fb13a1d059ecdc0b6e979ffec64b80371cf20a2088b0 /code/build-arm64.sh
```

The prebuilt binaries are then compressed using `upx` version 4.2.4. Expected sha256 checksums are available along with the links to the prebuilt binaries.

## ip-to-vsock

The ip-to-vsock proxy listens on a fixed IP address and proxies any incoming connections to a fixed vsock address.

### Prebuilt binaries

amd64: https://artifacts.marlin.org/oyster/binaries/ip-to-vsock_v1.0.0_linux_amd64 \
checksum: 9ee610acc9fb3ceaa446732a0c93c892c75e8e339dde955fdc205ba6ac290154

arm64: https://artifacts.marlin.org/oyster/binaries/ip-to-vsock_v1.0.0_linux_arm64 \
checksum: 8fcc4c7484ee703fe5d0eabe45a4e987941ae258d324318d70220b0836879300

### Usage

```bash
$ ./target/release/ip-to-vsock --help
Usage: ip-to-vsock --ip-addr <IP_ADDR> --vsock-addr <VSOCK_ADDR>

Options:
  -i, --ip-addr <IP_ADDR>        ip address of the listener side (e.g. 0.0.0.0:4000)
  -v, --vsock-addr <VSOCK_ADDR>  vsock address of the upstream side (e.g. 88:4000)
  -h, --help                     Print help
  -V, --version                  Print version
```

## vsock-to-ip

The vsock-to-ip proxy listens on a fixed vsock address and proxies any incoming connections to a fixed ip address.

### Prebuilt binaries

amd64: https://artifacts.marlin.org/oyster/binaries/vsock-to-ip_v1.0.0_linux_amd64 \
checksum: 8ad67e28b18a742c3b94078954021215b57a287ee634f09556efabcac0b99597

arm64: https://artifacts.marlin.org/oyster/binaries/vsock-to-ip_v1.0.0_linux_arm64 \
checksum: c55bd946a100f8e49b75c46e2e5d4bbb6be134e2f35b0d0927afeeca55fba5d0

### Usage

```bash
$ ./target/release/vsock-to-ip --help
Usage: vsock-to-ip --vsock-addr <VSOCK_ADDR> --ip-addr <IP_ADDR>

Options:
  -v, --vsock-addr <VSOCK_ADDR>  vsock address of the listener side (e.g. 88:4000)
  -i, --ip-addr <IP_ADDR>        ip address of the listener side (e.g. 127.0.0.1:4000)
  -h, --help                     Print help
  -V, --version                  Print version
```

## ip-to-vsock-transparent

The ip-to-vsock-transparent proxy listens on a fixed IP address and proxies any incoming connections to a fixed vsock address much like the [ip-to-vsock](#ip-to-vsock) proxy. The key difference being, it fetches the original destination of the connection using `SO_ORIGINAL_DST` and sends it on the vsock connection first before acting as a simple proxy. Meant to be used in conjunction with [vsock-to-ip-transparent](#vsock-to-ip-transparent) proxy and iptables rules to intercept outgoing connections.

### Prebuilt binaries

amd64: https://artifacts.marlin.org/oyster/binaries/ip-to-vsock-transparent_v1.0.0_linux_amd64 \
checksum: 15ecdf4ed7c0a3f65ebfa2fb10f0c1cb60e67677162db8cca6915aabb5afd4b9

arm64: https://artifacts.marlin.org/oyster/binaries/ip-to-vsock-transparent_v1.0.0_linux_arm64 \
checksum: 4a1beedb1a956e350ab38d52d3bfb557aff37562a10c7f42ca394c0e2f574a7e

### Usage

```bash
$ ./target/release/ip-to-vsock-transparent --help
Usage: ip-to-vsock-transparent --ip-addr <IP_ADDR> --vsock-addr <VSOCK_ADDR>

Options:
  -i, --ip-addr <IP_ADDR>        ip address of the listener side (e.g. 127.0.0.1:1200)
  -v, --vsock-addr <VSOCK_ADDR>  vsock address of the upstream side, usually the other side of the transparent proxy (e.g. 3:1200)
  -h, --help                     Print help
  -V, --version                  Print version
```

## vsock-to-ip-transparent

The vsock-to-ip-transparent proxy listens on a fixed vsock address and proxies any incoming connections to a dynamic IP address much like the [vsock-to-ip](#vsock-to-ip) proxy except the destination address is not fixed. The destination address is obtained from the beginning of the stream after which it connects to the destination and acts as a simple proxy. Meant to be used in conjunction with [ip-to-vsock-transparent](#ip-to-vsock-transparent) proxy.

### Prebuilt binaries

amd64: https://artifacts.marlin.org/oyster/binaries/vsock-to-ip-transparent_v1.0.0_linux_amd64 \
checksum: 0a280f2bc85007e115be6f6e906eebc50c671e402d6fa5f7d531b0ad77c11ccd

arm64: https://artifacts.marlin.org/oyster/binaries/vsock-to-ip-transparent_v1.0.0_linux_arm64 \
checksum: 4ac1f337b1239d445b85c0dc3904ba57aec0f3d816b4d8120c26f5e0af0acfc1

### Usage

```bash
$ ./target/release/vsock-to-ip-transparent --help
Usage: vsock-to-ip-transparent --vsock-addr <VSOCK_ADDR>

Options:
  -v, --vsock-addr <VSOCK_ADDR>  vsock address of the listener side, usually open to the other side of the transparent proxy (e.g. 3:1200)
  -h, --help                     Print help
  -V, --version                  Print version
```

## port-to-vsock-transparent

The port-to-vsock-transparent proxy listens on a fixed IP address and proxies any incoming connections to a dynamic vsock address much like the [ip-to-vsock](#ip-to-vsock) proxy except the destination port is not fixed. The proxy fetches the original destination of the connection using `SO_ORIGINAL_DST` and proxies it to the vsock on the same port. Meant to be used in conjunction with iptables rules to intercept incoming connections.

### Prebuilt binaries

amd64: https://artifacts.marlin.org/oyster/binaries/port-to-vsock-transparent_v1.0.0_linux_amd64 \
checksum: 4243a018123ead1f8c35d604b8455a64917dc90df34a2865a96a484ee8009bc8

arm64: https://artifacts.marlin.org/oyster/binaries/port-to-vsock-transparent_v1.0.0_linux_arm64 \
checksum: 565c188eaa8996e4204a7934e6c963a9908ff8fa6570cb4d5572dfee09c58059

### Usage

```bash
$ ./target/release/port-to-vsock-transparent --help
Usage: port-to-vsock-transparent --ip-addr <IP_ADDR> --vsock <VSOCK>

Options:
  -i, --ip-addr <IP_ADDR>  ip address of the listener side (e.g. 127.0.0.1:1200)
  -v, --vsock <VSOCK>      vsock address of the upstream side (e.g. 88:1200)
  -h, --help               Print help
  -V, --version            Print version
```

## Credits

All of them are based on proxies from [tokio](https://tokio.rs/), with the transparent proxies also borrowing from [linkerd](https://linkerd.io/) for `SO_ORIGINAL_DST` related functionality.
