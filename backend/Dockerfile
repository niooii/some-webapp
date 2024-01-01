# build

FROM rust:1.75.0 as builder

WORKDIR /app

COPY . .

RUN cargo build

# production 

FROM ubuntu:latest

COPY --from=builder /app/target/debug/main-api /

CMD ["./main-api"]