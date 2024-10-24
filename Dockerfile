FROM rust:1.48

WORKDIR /usr/src/rust-simple-webserver
COPY . .

RUN cargo install --path general

CMD ["general"]
