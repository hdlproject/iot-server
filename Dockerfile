FROM rust:1.82

ARG APP_NAME

WORKDIR /usr/src/iot-server
COPY . .

RUN cargo install --path ${APP_NAME}

RUN mv /usr/local/cargo/bin/${APP_NAME} /usr/local/cargo/bin/app

CMD ["app"]
