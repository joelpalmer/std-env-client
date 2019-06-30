use std::env;

fn main() {
    println!("Inspecting the processes's environment...");
    println!("Here are the constants associated with the current target:");
    print_constants();
}

fn print_constants() {
    let arch = env::consts::ARCH;
    println!("Architecture of the current CPU is: {}", arch);

    let dll_extension = env::consts::DLL_EXTENSION;
    println!(
        "file extension used for shared libraries on this platform: {}",
        dll_extension
    );

    let dll_prefix = env::consts::DLL_PREFIX;
    println!(
        "Filename prefix used for shared libraries on this platform: {}",
        dll_prefix
    );

    let dll_suffix = env::consts::DLL_SUFFIX;
    println!(
        "Filename suffix used for shared libraries on this platform: {}",
        dll_suffix
    );

    let exe_extension = env::consts::EXE_EXTENSION;
    println!(
        "File extension, if any, used for executable binaries on this platform: {}",
        exe_extension
    );

    let exe_suffix = env::consts::EXE_SUFFIX;
    println!(
        "File extension, if any, used for executable binaries on this platform: {}",
        exe_suffix
    );

    let os_family = env::consts::FAMILY;
    println!("Operating system family: {}", os_family);

    let operating_system = env::consts::OS;
    println!("Specific Operating system in use: {}", operating_system);
}
