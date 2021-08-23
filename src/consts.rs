use serde::{Deserialize, Serialize};

#[derive(PartialEq, Hash, Clone, Default, Deserialize, Serialize)]
pub struct ConnectedUsbDevices {
    pub vendor: Option<String>,
    pub description: Option<String>,
    pub serial_number: Option<String>,
    pub volume_label: Option<String>,
    pub filesystem: Option<String>,
}
