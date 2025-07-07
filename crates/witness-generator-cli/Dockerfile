# Build the witness-generator
FROM rust:1.85 AS builder
RUN apt-get update && apt-get install -y build-essential libclang-dev
WORKDIR /usr/src/zkevm-benchmark-workload
COPY . .
RUN cargo build --release --package witness-generator

# Create the final image
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && update-ca-certificates
COPY --from=builder /usr/src/zkevm-benchmark-workload/target/release/witness-generator /usr/local/bin/witness-generator
ENTRYPOINT ["/usr/local/bin/witness-generator-cli"]
