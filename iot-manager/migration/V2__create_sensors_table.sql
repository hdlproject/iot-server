CREATE TABLE sensors
(
    id          SERIAL PRIMARY KEY,
    device_id   VARCHAR          NOT NULL,
    temperature DOUBLE PRECISION NOT NULL,
    humidity    DOUBLE PRECISION NOT NULL,
    FOREIGN KEY (device_id) REFERENCES devices (id)
)
