use std::process::Command;

fn main() {
    Command::new("clear")
        .output()
        .expect("failed to execute 'clear' process");

    println!("__________ buil.rs__________");
}
