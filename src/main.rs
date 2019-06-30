use std::env;

fn main() {
    println!("Inspecting the processes's environment...");
    println!("Here are the constants associated with the current target:");
    print_constants();
}

fn print_constants() {
    let arch = env::consts::ARCH;
    println!("Architecture of the current CPU is {}", arch);
}
