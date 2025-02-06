use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DevicePayload {
    home: String,
    room: String,
    name: String,
}
