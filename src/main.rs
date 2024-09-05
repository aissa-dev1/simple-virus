use std::{
    fs::File,
    io::{self, Write},
    process::Command,
};

const APP_NAME: &str = "Simple Virus";
const APP_VERSION: f32 = 0.1;

fn main() {
    let single_run: bool = false;
    let max_iterations: u64 = 10;

    welcome();
    virus_logic(single_run, max_iterations);
    input_close();
}

fn welcome() {
    println!("Welcome to {} v{}.", APP_NAME, APP_VERSION);
}

fn virus_logic(single_run: bool, max_iterations: u64) {
    let mut index: u64 = 1;

    if single_run {
        spawn_cmd_window(index);
        let _ = generate_copy_file(index);
        return;
    }

    loop {
        if index > max_iterations {
            break;
        }

        spawn_cmd_window(index);
        let _ = generate_copy_file(index);

        index += 1;
    }
}

fn spawn_cmd_window(code: u64) {
    Command::new("cmd")
        .args(&["/C", "start", "cmd"])
        .spawn()
        .expect("Execute command start failed!");

    println!("Execute command start {}", code);
}

fn generate_copy_file(code: u64) -> io::Result<()> {
    let file_name_format = format!("copy-{}.txt", code);
    let file_content_format = format!("This is a copy created by {} code {}", APP_NAME, code);
    let mut file = File::create(file_name_format.as_str())?;

    file.write_all(file_content_format.as_str().as_bytes())?;

    println!("Create copy file {}", code);

    Ok(())
}

fn input_close() {
    println!("Press Enter to exit...");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
}
