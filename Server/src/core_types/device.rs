use crate::core_types::DeviceID;

#[derive(Debug, Clone, serde::Deserialize)]
#[allow(dead_code)]
pub struct Device {
    id: Option<DeviceID>,
    name: String,
    is_active: bool,
    is_private_session: bool,
    is_restricted: bool,
    #[serde(rename = "type")]
    device_type: DeviceType,
    #[serde(rename = "volume_percentage")]
    volume: Option<u8>,
    supports_volume: bool,
}

impl Device {
    pub fn id(&self) -> Option<&DeviceID> {
        self.id.as_ref()
    }
    pub fn is_active(&self) -> bool {
        self.is_active
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl PartialEq for Device {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Debug, Clone, Copy, serde::Deserialize, PartialEq, Eq)]
pub enum DeviceType {
    #[serde(rename = "computer")]
    Computer,
    #[serde(rename = "smartphone")]
    SmartPhone,
    #[serde(rename = "speaker")]
    Speaker,
    #[serde(other)]
    Other,
}
