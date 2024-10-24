use tokio_postgres::{Client, NoTls, Error};

pub struct Device {
    pub id: String,
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

    pub async fn create(&mut self, device: &Device) -> Result<(), Error> {
        self.client.execute(
            "INSERT INTO devices (id, lon, lat) VALUES ($1, $2, $3);",
            &[&device.id, &device.lon, &device.lat],
        ).await?;

        Ok(())
    }

    pub async fn get(&mut self, id: &str) -> Result<Device, Error> {
        let row = self.client.query_one(
            "SELECT id, lon, lat FROM devices WHERE id = $1;",
            &[&id],
        ).await?;

        let device = Device {
            id: row.get(0),
            lon: row.get(1),
            lat: row.get(2),
        };

        Ok(device)
    }
}
