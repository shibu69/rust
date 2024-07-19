use std::fs::File;
use std::io::{self, Read};

// declarative macros

macro_rules! vec {
    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();

        $(
            temp_vec.push($x);
        )*

        temp_vec
        }
    };
}

macro_rules! subtract{
    ($a : expr,$b :expr) =>{
        {
            $a - $b
        }
    }
}

fn read_file_content(file_path: &str) -> Result<String, io::Error> {
    let mut my_file = File::open(file_path)?;

    let mut content = String::new();

    my_file.read_to_string(&mut content)?;

    Ok(content)
}

pub fn er_handler() {
    match read_file_content("src/Example.txt") {
        Ok(content) => println!("content is : {}", content),
        Err(err) => println!("Got Error : {}", err),
    }

    //declarative macros

    let new_vec = vec!([1, 2, 3]);
    println!("Vector {:?}",new_vec);

    // the above code is doing the same thing as new_vec.push(1),new_vec.push(2),new_vec.push(3) but in this case we created our own vector macro

    let result = subtract!(1555688,45485);
    println!("the result is {}",result);
}
