use colored::Colorize;
use dialoguer::Select;
use crate::models::{DeviceType, GpuType, UserChoices};
use crate::utils::sleep;

pub fn get_user_choices() -> UserChoices {
    println!("{}", "Welcome to ArchTry - Arch Linux Installation Simulator!\n".bright_green());
    sleep(2);
    let gpu_type = get_gpu_type();
    let device_type = get_device_type();
    UserChoices::new(gpu_type, device_type)
}

fn get_gpu_type() -> GpuType {
    let options = &["AMD", "Intel", "NVIDIA"];
    let selection = Select::new()
        .with_prompt("Choose your GPU type")
        .items(&options[..])
        .interact()
        .unwrap();

    match selection {
        0 => GpuType::Amd,
        1 => GpuType::Intel,
        2 => GpuType::Nvidia,
        _ => panic!("Invalid selection"),
    }
}

fn get_device_type() -> DeviceType {
    let options = &["Laptop", "PC"];
    let selection = Select::new()
        .with_prompt("Do you want a laptop or PC?")
        .items(&options[..])
        .interact()
        .unwrap();

    match selection {
        0 => DeviceType::Laptop,
        1 => DeviceType::Pc,
        _ => panic!("Invalid selection"),
    }
}
