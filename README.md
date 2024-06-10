# Rust Playground

This is a litte example for a containerized rust program with a network interface.

## Run Local

```bash
cargo build --release
/target/release/app
```
Access http://127.0.0.1:80/ in your browser.

## Run Container

```bash
podman build . -t test
podman run -p 8080:80 -d --name test localhost/test:latestpodman 
```

Access http://127.0.0.1:8080/ in your browser.