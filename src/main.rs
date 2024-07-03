struct Mytuples(bool, u32, f32); // if we have multiple data with same tuple we can do this

//structs can be made in as

struct Student {
    is_student: bool,
    class: u32,
    height: u32,
}

struct Config {
    port: u32,
}

//lets create some enum

// enum Directions{
//     North,
//     South,
//     East,
//     West
// }

fn main() {
    // let d = Directions::South;

    let mytuple = Mytuples(true, 32, 67.3); //using struct template for tuples
    println!("{}", mytuple.0);
    println!("{}", mytuple.1);
    println!("{}", mytuple.2);

    //now we can use struct as
    let student1 = Student {
        is_student: true,
        height: 110,
        class: 6,
    };
    println!("{}", student1.height); //to get value out of the struct
    println!("{}", student1.is_student);
    println!("{}", student1.class);

    let student_tuple: (bool, i32, char) = (true, 69, 'a'); //creating single tupple
    println!("{}", student_tuple.2);

    let array: [u32; 5] = [1, 2, 3, 4, 5];
    for i in 0..array.len() {
        // for loop
        println!("element at {} index is {}", i, array[i]);
    }
    let slice = &array[1..3]; //slice in array
    for i in 0..slice.len() {
        println!("element at{} index in slice is {}", i, slice[i]);
    }

    let str = "Hello world";
    let strslice = &str[2..7]; //slice in string
    println!("{}", strslice);

    let val = 10;
    let r1 = &val; //immutable reference example
    let r2 = &val; //immutable reference example
    println!("{r1} should be the same as {r2}.");

    let mut config: Config = Config { port: 8000 };

    let new_config: &mut Config = &mut config;
    new_config.port = 9000;

    // println!("{}", config.port);          // we cant directly access the original variable untill the reference variable is in scope
    println!("{}", new_config.port);

    let mut str1 = "Hello World";
    let str2 = &mut str1;
    *str2 = "Namaste World";

    println!("{str2}");
    println!("{str1}");

    // Derefrencing 

    let mut var1= 36;
    let reference_var = &mut var1;
    *reference_var =64;
    // let var2 = *reference_var;

    println!("{}",var1);
    // println!("{}",var2);

    let _x=9;
    let _x=10;
    println!("{_x}");
    let (x, y) = (2, 3);
    println!("{x},{y}");
    
    //destruct struct

    let config2 = Config{
        port:3000,
    };

    println!("{}",config2.port);

    let Config{port}=config2;

    println!("{}",config2);


}
