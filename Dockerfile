FROM rust:latest AS compiler
WORKDIR /build
COPY . .
RUN cargo build --release

FROM ubuntu:latest as runtime
WORKDIR /runtime
COPY --from=compiler /build/target .
CMD ["/runtime/release/app"]
