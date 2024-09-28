use std::io;
mod zap;

fn print_logo() {
    let logo: Vec<_> = [
        "",
        " ░▒▓████████▓▒░  ░▒▓██████▓▒░  ░▒▓███████▓▒░  ",
        "        ░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░ ",
        "      ░▒▓██▓▒░  ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░ ",
        "    ░▒▓██▓▒░    ░▒▓████████▓▒░ ░▒▓███████▓▒░  ",
        "  ░▒▓██▓▒░      ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░        ",
        " ░▒▓█▓▒░        ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░        ",
        " ░▒▓████████▓▒░ ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░        ",
        ""
    ].into_iter().collect::<Vec<&str>>();

    for line in logo {
        println!("{}", line);
    }
}

fn print_welcome_message() {
    println!(
        "Zap{reset}, is your Rust{reset} project manager!{reset}",
        reset = "\x1B[0m",
    );

    println!(
        "Type 'zap --help' to see all the options available.{reset}",
        reset = "\x1B[0m",
    );
}

fn listener(input: &str) {
    if input == "zap --help" {
        println!("1.    zap --help                # displays all of the commands.");
        println!("2.    zap new                   # creates a new project.");
        println!("3.    zap run                   # runs the project.");
    } else if input == "zap new" {
        println!("Enter a project name: ");
        let mut project_name = String::new();
        io::stdin().read_line(&mut project_name).unwrap();
        project_name = project_name.trim().to_string();
        zap::create_project(&project_name);
    }
    else if input == "zap run" {
        zap::run_project();
    }
    else {
        println!("Invalid command!");
    }
}

fn main() {
    print_logo();
    print_welcome_message();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!("");
    listener(&input.trim());
}
