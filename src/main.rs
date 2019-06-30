use std::env;

fn main() {
    println!("Inspecting the processes's environment...");
    println!("Here are the Constants associated with the current target:");
    print_constants();
    println!("Here are the functions in action:");
    call_functions();
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

fn call_functions() {
    // args() -> Args - use args_os() for non-unicode
    println!("args():");
    for arg in env::args() {
        println!("Arg: {}", arg)
    }

    // args_os() -> ArgsOs - use args_os() for non-unicode
    println!("args_os():");
    for arg in env::args_os() {
        println!("Arg (OS): {:?}", arg)
    }

    // current_dir() -> Result<PathBuf>
    println!("current_dir():");
    let path = env::current_dir();
    match path {
        Ok(val) => println!("Current Directory is: {:?}", val),
        Err(e) => println!("Current Directory value is invalid: {}", e),
    }

    // var<K: AsRef<OsStr>>(key: K) -> Result<String, VarError>
    println!("var():");
    let key = "HOME";
    match env::var(key) {
        Ok(val) => println!("The key {} has a value of {:?}", key, val),
        Err(e) => println!("couldn't interpret {}: {}", key, e),
    }

    // current_exe() -> Result<PathBuf>
    println!("current_exe():");
    match env::current_exe() {
        Ok(exe_path) => println!("The path to the current exe is {:?}", exe_path),
        Err(e) => println!("Failed to get current exe path: {}", e),
    }
}
