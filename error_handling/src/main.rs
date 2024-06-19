use std::fs::File;
use std::io::{self, ErrorKind, Read};
fn main() {
    let greeting_file_result = File::open("hello_txt");
    //Using a match statement
    let greeting = match greeting_file_result {
        Ok(file) => file,
        Err(err) => {
            match err.kind() {
                ErrorKind::NotFound => {
                    match File::create("hello.txt") {
                        Ok(f) => f,
                        Err(err) => panic!("Problem Creating the file: {:?}", err)
                    }
                }

                _ => { panic!("Problem opening the file")}
            }
        }
    };


    //Using closure
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|err| {
                panic!("Problem creating the file: {:?}", err);
            })
        }else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e)
    }
}

fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut username_file = File::open("Hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn shorter_read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}