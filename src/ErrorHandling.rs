use std::fs::File;
use std::io::{self, Read};

pub fn er_handler() {
    match read_file_content("src/Example.txt") {
        Ok(content) => println!("content is : {}", content),
        Err(err) => println!("Got Error : {}", err),
    }
}

fn read_file_content(file_path: &str) -> Result<String, io::Error> {
    let mut my_file = File::open(file_path)?;

    let mut content = String::new();

    my_file.read_to_string(&mut content)?;

    Ok(content)
}
