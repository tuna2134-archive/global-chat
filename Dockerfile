FROM rust AS builder

WORKDIR /src/builder

RUN apt-get update && apt-get install -y musl-tools libssl-dev pkg-config
RUN rustup target add x86_64-unknown-linux-musl

COPY . .
RUN cargo build --target=x86_64-unknown-linux-musl --release

FROM scratch

WORKDIR /src/app

COPY --from=builder /src/builder/target/x86_64-unknown-linux-musl/release/global-chat .

CMD ["./global-chat"]
