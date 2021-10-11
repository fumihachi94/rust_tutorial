use std::fs::File;
use std::io::stdin;
use std::io::ErrorKind;
use std::io::{stdout, Write};

fn main() {
    println!(
        "
    =================================
    Tutorial : Error handling
    - 1. Unrecoverable Erorr Hnadling
    - 2. Recoverable Error Handling
    ================================="
    );

    print!("Please select number> ");
    stdout().flush().unwrap();
    let mut buf = String::new();
    stdin().read_line(&mut buf).ok();
    let select: i32 = buf.trim().parse().unwrap();

    match select {
        1 => unrecoverable_error_handling(),
        2 => recoverable_error_handling(),
        _ => panic!("Please select 1 or 2."),
    }
}

fn unrecoverable_error_handling() {
    let _f = match File::open("hello.txt") {
        Ok(file) => file,
        Err(err) => panic!("Problem opening the file {:?}", err),
    };
}

fn recoverable_error_handling() {
    let _f = match File::open("hello.txt") {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => {
                    println!("Created new file at current directory.");
                    fc
                }
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    println!("{:?}", _f);

    let _r = match std::fs::remove_file("hello.txt") {
        Ok(remove) => {
            println!("Delete the file.");
            remove
        },
        Err(err) => panic!("Problem opening the file {:?}", err),
    };
}
