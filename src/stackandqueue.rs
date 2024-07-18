use std::io;
use std::collections::HashMap;

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

    let arrays: [[u8; 5]; 2] = [[1,2,3,4,5],[6,7,8,9,10]];

    for i in 0..arrays.len(){
        for j in 0..arrays[i].len(){
            println!("{}",arrays[i][j]);
        }
    }

    let mut v1 = vec![1,2,3,4,5];

    for i in &mut v1{
        *i+=1
    }

    println!("{:?}",v1);

    let user_list : Vec<(&str,u32)>=vec![
        ("Sourabh",10000),
        ("Kusum",60000),
        ("Harsh",300000),
        ("Shivendra",50000),
    ];

    let user_map: HashMap<&str, u32> = user_list.into_iter().collect();

    println!("{:?}",user_map);




    
}