#[cfg(test)]
use crate::device::model::{DeviceRepo, Device};

#[test]
fn test_device_repo_create() {
    let mut device_repo = DeviceRepo::new("postgres://admin:admin@localhost:5433/iot-manager").unwrap();

    let result = device_repo.create(Device {
        id: "test-id",
        lon: 80.80,
        lat: 10.10,
    });

    assert!(result.is_ok());
}
