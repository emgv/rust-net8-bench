FROM rust:1.87

WORKDIR /usr/src/rs-web-api
COPY . .

RUN cargo build --release

CMD ["./target/release/rs-web-api"]


#FROM rust:1.87 as builder
#WORKDIR /usr/src/rs-web-api
#COPY . .
#RUN cargo build --release

#FROM debian:bookworm-slim
#COPY --from=builder /usr/src/rs-web-api/target/release/rs-web-api /usr/local/bin/rs-web-api
#CMD ["rs-web-api"]
