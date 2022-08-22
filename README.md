# sparsesrv

A high performance sparse file webserver written in Rust

## Why?

We needed something to serve large, random files fast for a speedtest service.
In the past we had prepared files on the filesystem. That was slow and took up a lot of space. When deploying an anycasted speedtest service we didn't want to waste the space on 8 machines - so this project was born.

### Why don't you deliver just zeroes?

Some proxies compress or cache the transmitted data. We want to test the end to end speed. That's why we're using a random number generator.

The data is generated using the extremely fast [SmallRng](https://docs.rs/rand/0.5.1/rand/rngs/struct.SmallRng.html) of the [rand crate](https://docs.rs/rand/0.5.1/rand/).

## How fast is it?

In our setup running in VMs on AMD EPYC 7302P processors @ 3.3 GHz we achieve ~20 Gbit/s per stream.
We're sure further optimization is possible, but this is okay for us, as our machines only have 25 Gbit/s interfaces anyways. Feel free to contribute optimizations.

## Build

If you are using the Nix package manager, there is a flake file included. ❄️

Building manually is straightforward using [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html):
```
cargo build --release
```

## Deployment Considerations

### Network Stack
Tune your kernel network stack:
```
# Increase network buffer size
net.core.rmem_max=134217728
net.core.wmem_max=134217728

# Increase TCP buffer size
net.ipv4.tcp_rmem = 4096 87380 67108864
net.ipv4.tcp_wmem = 4096 65536 67108864

# Enable BBR TCP scheduler
net.core.default_qdisc=fq
net.ipv4.tcp_congestion_control=bbr
```

Consider [increasing your NICs buffer sizes](https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/8/html/configuring_and_managing_networking/monitoring-and-tuning-the-rx-ring-buffer_configuring-and-managing-networking) and the [amount of queues](https://www.linode.com/docs/guides/multiqueue-nic/).
If you are running this in a VM, consider [enabling Multi-Queue in your virtio-net](https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/7/html/virtualization_tuning_and_optimization_guide/sect-virtualization_tuning_optimization_guide-networking-techniques#sect-Virtualization_Tuning_Optimization_Guide-Networking-Multi-queue_virtio-net)

### Reverse Proxy

If you are using nginx as a reverse proxy you want to consider a few settings:

* Set caching and last-modified headers
* Turn off gzip compression
* Increase `proxy_read_timeout`
* Increase `proxy_buffers` size

That's what our config looks like:
```
add_header Cache-Control 'no-store, no-cache, max-age=0, no-transform';
add_header Last-Modified $date_gmt;
  if_modified_since off;
  expires off;
  etag off;

gzip off;
proxy_read_timeout 999;
proxy_buffers 16 128k;
```
