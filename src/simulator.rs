use colored::Colorize;
use std::io::{self, Write};
use crate::models::{DeviceType, GpuType, UserChoices};
use crate::utils::{sleep, get_user_input, create_progress_bar, show_header, show_success, show_warning, get_user_input_with_prompt};

/// Run the full Arch Linux installation simulation
pub fn run_simulation(user_choices: &UserChoices) {
    show_header("Starting Arch Linux Installation Simulator...");
    sleep(3);
    show_header("Welcome to Arch Linux!");
    sleep(2);

    println!("\n{}", "Setting up system based on your choices:".bright_blue());
    println!("- Disk Size: 50GB (Fixed)");
    println!("- GPU Type: {}", user_choices.gpu_type);
    println!("- Device Type: {}", user_choices.device_type);
    sleep(1);

    // Check if the device is a PC and show LAN connection message
    if let DeviceType::Pc = user_choices.device_type {
        println!("\n{}", "Connected via LAN.".bright_green());
        sleep(1);
    }

    if let DeviceType::Laptop = user_choices.device_type {
        simulate_wifi_setup();
    }

    simulate_partitioning();
    simulate_mount();
    simulate_pacstrap(&user_choices.gpu_type);
    simulate_final_steps();
}

/// Simulate Wi-Fi setup for laptops
fn simulate_wifi_setup() {
    show_header("Setting up Wi-Fi for laptop...");
    show_warning("Note: You need to manually connect to Wi-Fi.");

    simulate_command("ip a", "Show network interfaces", false, || {
        println!(
            r#"
1: wlan0: <NO-CARRIER,BROADCAST,MULTICAST,UP> mtu 1500 qdisc noqueue state DOWN group default qlen 1000
    link/ether XX:XX:XX:XX:XX:XX brd ff:ff:ff:ff:ff:ff
"#
        );
    });

    simulate_command("iwctl station wlan0 get-networks", "Scan for Wi-Fi networks", false, || {
        println!(
            r#"
                               Available networks
--------------------------------------------------------------------------------
      Network name                  Security            Signal
--------------------------------------------------------------------------------
      archtry_wifi                    NULL               ****
"#
        );
    });

    simulate_command("iwctl station wlan0 connect archtry_wifi", "Connect to Wi-Fi", false, || {
        println!("\nConnected to archtry_wifi.\n");
    });

    sleep(1);
}

/// Simulate disk partitioning
fn simulate_partitioning() {
    show_header("Partitioning disk...");
    show_warning("Note: You need to manually partition the disk.");

    simulate_command("lsblk", "Show disk layout", false, || {
        println!(
            r#"
NAME        MAJ:MIN RM   SIZE RO TYPE MOUNTPOINTS
sdz         259:0   0   50.0G  0 disk
"#
        );
    });

    simulate_command("fdisk /dev/sdz", "Create partitions", false, || {
        println!("\nCreating partitions...\n");
    });

    simulate_command("mkfs.fat -F 32 /dev/sdz1", "Format EFI partition", false, || {
        println!("\nFormatting EFI partition...\n");
    });

    simulate_command("mkfs.ext4 /dev/sdz2", "Format root partition", false, || {
        println!("\nFormatting root partition...\n");
    });

    show_success("Disk partitioned and formatted.");
}

/// Simulate mounting partitions
fn simulate_mount() {
    show_header("Mounting partitions...");
    show_warning("Note: You need to manually mount the partitions.");

    simulate_command("mount /dev/sdz2 /mnt", "Mount root partition", false, || {
        println!("\nRoot partition mounted.\n");
    });

    simulate_command("mount --mkdir /dev/sdz1 /mnt/boot/efi", "Mount EFI partition", false, || {
        println!("\nEFI partition mounted.\n");
    });

    show_success("Partitions mounted.");
}

/// Simulate base system installation
fn simulate_pacstrap(gpu_type: &GpuType) {
    show_header("Installing base system...");
    show_warning("Note: You need to manually install the base system.");

    let base_packages = match gpu_type {
        GpuType::Amd => "amd-ucode",
        GpuType::Intel => "intel-ucode",
        GpuType::Nvidia => "nvidia",
    };

    simulate_command(
        &format!("pacstrap -K /mnt base base-devel linux linux-firmware {} grub", base_packages),
        "Install base system and GRUB",
        false,
        || {
            let total_steps = 10;
            let pb = create_progress_bar(total_steps);
            
            for i in 0..total_steps {
                pb.set_position(i as u64);
                pb.set_message("Installing base system and GRUB...");
                sleep(1);
            }
            
            pb.finish_with_message("Base system and GRUB installed successfully.");
            println!();
        },
    );

    show_success("Base system and GRUB installed.");
}

/// Simulate final installation steps
fn simulate_final_steps() {
    show_header("Finishing installation...");

    simulate_command("genfstab -U /mnt >> /mnt/etc/fstab", "Generate fstab", false, || {
        println!("\nfstab generated.\n");
    });

    // Enter chroot environment
    simulate_command("arch-chroot /mnt", "Chroot into the system", false, || {
        println!("\nChrooted into the system.\n");
    });

    // Inside chroot environment now
    let in_chroot = true;
    
    set_password("root", in_chroot);

    let username = get_user_input_with_prompt("\nEnter a username to create:", false);
    simulate_command(&format!("useradd -mG wheel {}", username), "Create a new user", in_chroot, || {
        println!("\nUser '{}' created and added to the wheel group.\n", username);
    });
    set_password(&username, in_chroot);

    choose_environment(in_chroot);

    // Configure GRUB before exiting chroot
    simulate_command(
        "grub-install --target=x86_64-efi --efi-directory=/boot/efi --bootloader-id=GRUB",
        "Install GRUB to the EFI partition",
        in_chroot,
        || {
            println!("\nGRUB installed to the EFI partition.\n");
        },
    );

    simulate_command(
        "grub-mkconfig -o /boot/grub/grub.cfg",
        "Generate GRUB configuration",
        in_chroot,
        || {
            println!("\nGRUB configuration generated.\n");
        },
    );

    simulate_command("exit", "Exit chroot environment", in_chroot, || {
        println!("\nExited from chroot.\n");
    });

    // Back to archiso environment
    simulate_command("umount -R /mnt", "Unmount partitions", false, || {
        println!("\nPartitions unmounted.\n");
    });

    simulate_command("reboot", "Reboot the system", false, || {
        println!("\nSystem rebooting...\n");
    });

    println!("\n{}", "Installation complete! You learned 40-60% of how to install Arch Linux manually!".bright_green());
    println!("{}", "This is a fake simulation. For a complete guide, visit:".bright_blue());
    println!("{}", "https://wiki.archlinux.org/title/Installation_guide".bright_blue());
}

/// Set a password for a user
fn set_password(username: &str, in_chroot: bool) {
    loop {
        let command = if username == "root" {
            "passwd"
        } else {
            &format!("passwd {}", username)
        };

        println!("{}", format!("Run '{}' to set the password for '{}':", command, username).bright_blue());
        println!(); // Add spacing
        println!("[hint] type: {}", command.bright_cyan());
        
        // Display appropriate prompt based on environment
        if in_chroot {
            print!("{}:{} {} ", "root".bright_red(), "/".bright_blue(), "#".bright_red());
        } else {
            print!("{}@{} {} {} ", "root".bright_red(), "archiso".normal(), "~".bright_green(), "#".bright_red());
        }
        
        io::stdout().flush().unwrap();
        
        let input = get_user_input();

        if input.contains(command) {
            let password = get_user_input_with_prompt("Enter new password:", true);
            let confirm_password = get_user_input_with_prompt("Confirm password:", true);

            if password == confirm_password {
                println!("{}", format!("Password set for '{}'.", username).bright_green());
                break;
            } else {
                println!("{}", "Passwords do not match. Try again.".red());
            }
        } else {
            println!("{}", "Error: Invalid command. Try again.".red());
        }
    }
}

/// Choose a desktop environment or window manager
fn choose_environment(in_chroot: bool) {
    let options = &["GNOME", "Plasma", "Hyprland"];
    let selection = dialoguer::Select::new()
        .with_prompt("Choose a desktop environment or window manager to install:")
        .items(options)
        .interact()
        .unwrap();

    let package = match selection {
        0 => "gnome",
        1 => "plasma",
        2 => "hyprland",
        _ => panic!("Invalid selection"),
    };

    simulate_command(&format!("pacman -S {}", package), &format!("Install {}", options[selection]), in_chroot, || {
        let total_steps = 7;
        let pb = create_progress_bar(total_steps);
        
        for i in 0..total_steps {
            pb.set_position(i as u64);
            pb.set_message(format!("Installing {}...", options[selection]));
            sleep(1);
        }
        
        pb.finish_with_message(format!("{} installed successfully.", options[selection]));
        println!();
    });

    println!("\n{} installed.\n", options[selection].bright_green());
    sleep(1);
}

/// Simulate a command execution with a description and custom output
fn simulate_command<F>(command: &str, description: &str, in_chroot: bool, action: F)
where
    F: FnOnce(),
{
    // Display the prompt with colors and add an extra line
    println!("# {}", description.bright_blue());
    println!(); // Add a blank line for better spacing
    println!("[hint] type: {}", command.bright_cyan());
    
    // Display appropriate prompt based on environment
    if in_chroot {
        print!("{}:{} {} ", "root".bright_red(), "/".bright_blue(), "#".bright_red());
    } else {
        print!("{}@{} {} {} ", "root".bright_red(), "archiso".normal(), "~".bright_green(), "#".bright_red());
    }
    
    io::stdout().flush().unwrap(); // Ensure the prompt is displayed immediately

    // Wait for user input
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // Check if the user typed the correct command
    if input.trim() == command {
        action(); // Execute the action
    } else {
        println!("{}", "Error: Invalid command. Try again.".red());
        println!("{}", "Hint: Make sure you type the command exactly as shown.".bright_cyan());
        simulate_command(command, description, in_chroot, action); // Retry
    }
}
