# build

FROM rust:1.75.0 as builder

WORKDIR /app

COPY . .

ENV DATABASE_URL=postgres://niooi:abcde@127.0.0.1:9432/postgres
# ENV PGHOST=db
# ENV PGUSER=niooi
# ENV PGPASSWORD=abcde
# ENV PGDATABASE=postgres
RUN cargo install sqlx-cli
RUN sqlx migrate run
RUN cargo build --release

# production 

FROM ubuntu:latest

COPY --from=builder /app/target/release/main-api /
COPY --from=builder /app/main-api/.env /

CMD ["./main-api"]