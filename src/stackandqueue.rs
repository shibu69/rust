use std::io;

pub fn stkndq() {

    let mut input:String = String::new();

    println!("Enter You name :");

    io::stdin().read_line(&mut input).expect("failed to read");

    println!("Your name is {}",input);

    let mut s = String::from("Hello ");

    s.push('I'); //Hello I
    s.push_str(" software developerr");//Hello I software developerr

    s.insert_str(7, " am ");//Hello I am software developerr
    s.insert(11, 'a');// Hello I ama

    s.pop();

    print!("{}",s);

    
}