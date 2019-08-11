FROM rust:1.36-slim as build

RUN USER=root cargo new --bin mandelbrot-rs
WORKDIR /mandelbrot-rs

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Dependencies cache
RUN cargo build --release && rm src/*.rs

COPY ./src ./src
RUN rm target/release/deps/mandelbrot* && cargo build --release

FROM debian:stretch-slim
COPY --from=build /mandelbrot-rs/target/release/mandelbrot .
ENTRYPOINT ["./mandelbrot"]
