use tokio_postgres::{Client, NoTls, Error};

pub struct Device<'a> {
    pub id: &'a str,
    pub lon: f64,
    pub lat: f64,
}

pub struct DeviceRepo {
    client: Client,
}

impl DeviceRepo {
    pub async fn new(dsn: &str) -> Result<Self, Error> {
        let (client, connection) = tokio_postgres::connect(dsn, NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Connection error: {}", e);
            }
        });

        Ok(Self {
            client,
        })
    }

    pub async fn create<'a>(&mut self, device: Device<'a>) -> Result<(), Error> {
        self.client.execute(
            "INSERT INTO devices (id, lon, lat) VALUES ($1, $2, $3);",
            &[&device.id, &device.lon, &device.lat],
        ).await?;

        Ok(())
    }
}
