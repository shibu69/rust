use std::{collections::HashSet, rc::Rc};

struct UserDetails {
    name: String,
    age: u32,
}

struct Light {
    on:bool,
}

impl Light {
    fn new()->Self{
       Light{on:false}
    }

    fn light_on(&mut self){
        if !self.on{
           self.on =true;
           println!("Light is on");
        }
    }

    fn light_off(&mut self){
        if self.on{
            self.on = false;
            println!("Ohh the light is off")
        }
    }
}

impl Drop for Light {
    fn drop(&mut self) {
        self.light_off();
    }
}

struct Person {
    can_read :Rc<Light>,
}

impl Person {
    fn read_book(&mut self){
        if self.can_read.on{
            println!("wooo What a nice book");
        }

        else{
            println!("I can't read it , too dark here");
        }
    }
}



pub fn sequence() {
    //Sequences

    let arr: [u8; 5] = [1, 2, 3, 4, 5];
    let vector: Vec<u8> = vec![1, 2, 3, 4];

    println!("{}", arr[3]);
    println!("{:?}", vector[3]);

    //the vector variable is shorthand for

    let mut a: Vec<u8> = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);
    a.push(4);
    a.push(5);

    //or we can do

    println!("{:?}", a.pop());

    //Map in rust

    let user = [
        UserDetails {
            name: "Shivendra".to_string(),
            age: 20,
        },
        UserDetails {
            name: "Ashish".to_string(),
            age: 30,
        },
        UserDetails {
            name: "Sumit".to_string(),
            age: 50,
        },
    ];

    let age_of_sumit = user
        .iter()
        .find(|profile| profile.name == "Sumit")
        .unwrap()
        .age; // Not good approach

    println!("{age_of_sumit}");

    //HashMap and BtreeMap

    let name_to_profile: std::collections::HashMap<String, UserDetails> = user
        .into_iter()
        .map(|profile| (profile.name.clone(), profile))
        .collect();

    println!("{:?}", name_to_profile["Sumit"].age);

    //HashSets and BtreeSets

    let mut numbers = HashSet::from([2,3,4,5,6]);

    numbers.insert(8);

    println!("{:?}", &numbers);

    //Textual Types

    //1 String

    let mut one_piece = String::from("Monke");

    one_piece.push('y');
    one_piece.push_str(" D Luffy");

    println!("{one_piece}");
    println!("{:?}", "😊helloo".as_bytes());

    //2 Vector

    let mut vector_score = vec![90,60,50,70];

    vector_score.push(80);
    vector_score.pop();
     
    println!("{:?}",vector_score);

    //Smart Pointers

    // 1 Box<T>

    let box_sc = Box::from(10);
    println!("{box_sc}");
    let data = create_data(true);
    println!("{data:?}");

    // 2 Rc<T> & ArC<T>

    let light = {
        let mut light = Light::new();
        light.light_on();
        // Place the light in an `Rc<T>`
        Rc::new(light)
    };

    let mut mick = Person {
        // Note that `Light` does not implement `Clone`. We are cloning the
        // smart pointer here, not the value contained within. It is considered
        // good practice to call the clone implementation `Rc::clone` explicitly
        // because it encodes the intent to clone the smart pointer, not the
        // value itself.
        can_read: Rc::clone(&light),
    };
    let mut anna = Person { can_read:light};
    mick.read_book();
    anna.read_book();


}

fn create_data(small:bool)->Box<[u8]>{
    if small {
        Box::new([1,2,3,4])
    }
    else{
        Box::new([1,2,3,4,5,6,7,8,9,10])
    }
}
