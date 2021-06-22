# Tranquility 

[![Checks](https://github.com/smallglitch/tranquility/actions/workflows/rust.yml/badge.svg)](https://github.com/smallglitch/tranquility/actions/workflows/rust.yml)
[![Dependency status](https://deps.rs/repo/github/smallglitch/tranquility/status.svg)](https://deps.rs/repo/github/smallglitch/tranquility)

Small ActivityPub server written in Rust

### **Disclaimer**

Tranquility is far from finished and therefore not ready to be used in any capacity yet  
Backwards incompatible changes might occur  

## Requirements

- **Rust** 1.50+  
- **PostgreSQL** (9.5+ should be fine)  
- **Git** (build-time dependency; see [`build.rs`](tranquility/build.rs))  

## Prebuilt binaries

Release binaries are built daily for Linux x86 and Linux ARMv7  

[**Nightly tag**](https://github.com/smallglitch/tranquility/releases/tag/nightly)

## Email confirmation

Tranquility can send confirmation emails to users before they can log into their accounts

Compile with the `email` feature to enable it

You also need to configure credentials to a mail server in the configuration file

## Markdown formatted statuses

Tranquility supports posts formatted with Markdown (the posts are parsed via `pulldown-cmark` directly when submitted to the API)

Compile with the `markdown` feature to enable it

## Custom memory allocators

Tranquility currently supports two custom memory allocators  

Use them by compiling the server with one of the following feature flags:

- `jemalloc`: Use `jemalloc` as the memory allocator
- `mimalloc`: Use `mimalloc` as the memory allocator

These features are mutually exclusive  
If more than one is activated, all selected allocators are compiled in the binary but neither will be actually used  

## Jaeger integration

Tranquility supports exporting the data logged via tracing to a jaeger instance  
To enable this feature, compile Tranquility with the `jaeger` feature flag

## Progress

Implementation progress is being tracked [here](https://github.com/smallglitch/tranquility/issues/17)

