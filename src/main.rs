use std::{fs::File, io::{BufReader, Read}, env};

use rbrainfuck::interpreter;

fn read_file(path: &String) -> String {
    let mut code_buffer = String::new();

    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            panic!("Could not open{}:{}", path, e);
        }
    };

    let mut file_buf = BufReader::new(file);
    file_buf.read_to_string(&mut code_buffer).unwrap();

    code_buffer
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        interpreter::interpret(&read_file(&args[1]));
    } else if args.len() == 1{
        println!("Please input command like about: <command path>");
    } else {
        println!("Not supported yet.");
    }
}
