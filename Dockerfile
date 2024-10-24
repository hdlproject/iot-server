FROM rust:1.48

WORKDIR /usr/src/iot-server
COPY . .

RUN cargo install --path general

CMD ["general"]
