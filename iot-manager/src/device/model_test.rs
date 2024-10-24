#[cfg(test)]
use crate::device::model::{DeviceRepo, Device};

#[tokio::test]
async fn test_device_repo_create() {
    let mut device_repo = DeviceRepo::new("postgres://admin:admin@localhost:5433/iot-manager").await.unwrap();

    let device =  Device {
        id: "test-id".to_string(),
        lon: 80.80,
        lat: 10.10,
    };

    let create_result = device_repo.create(&device).await;
    assert!(create_result.is_ok());

    let get_result = device_repo.get(&device.id).await;
    assert!(get_result.is_ok());
}
