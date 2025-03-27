use std::fmt;

#[derive(Debug)]
pub struct UserChoices {
    pub gpu_type: GpuType,
    pub device_type: DeviceType,
}

impl UserChoices {
    pub fn new(gpu_type: GpuType, device_type: DeviceType) -> Self {
        UserChoices { gpu_type, device_type }
    }
}

#[derive(Debug)]
pub enum GpuType {
    Amd,
    Intel,
    Nvidia,
}

impl fmt::Display for GpuType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GpuType::Amd => write!(f, "AMD"),
            GpuType::Intel => write!(f, "Intel"),
            GpuType::Nvidia => write!(f, "NVIDIA"),
        }
    }
}

#[derive(Debug)]
pub enum DeviceType {
    Laptop,
    Pc,
}

impl fmt::Display for DeviceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeviceType::Laptop => write!(f, "Laptop"),
            DeviceType::Pc => write!(f, "PC"),
        }
    }
}
