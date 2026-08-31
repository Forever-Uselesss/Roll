/*


 do the init here depending on feature flag

 and then init and attach lister for events like run() function for them devices. and then when app notifies the divice responds and vice versa.

 pub fn init(/* I2C bus, etc. */) -> Result<DeviceRegistry, InitError> {
    let mut devices = DeviceRegistry::new();

    for entry in SUPPORTED_DEVICES.iter() {
        match probe_and_initialize(entry) {
            Ok(device) => {
                devices.push(device)?;
            }

            Err(DeviceError::NotPresent) => {
                // Optional device — continue
            }

            Err(error) => {
                // Handle an actual initialization failure
            }
        }
    }

    Ok(devices)
}



*/
