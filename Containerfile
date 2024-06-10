FROM rust:latest AS compiler
WORKDIR /build
COPY . .
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM scratch as runtime
WORKDIR /runtime
COPY --from=compiler /build/target/x86_64-unknown-linux-musl/release/app .
CMD ["/runtime/app"]
