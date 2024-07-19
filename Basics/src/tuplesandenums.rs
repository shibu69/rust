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

pub fn tuplesandenums() {
    // let d = Directions::South;

    let mytuple = Mytuples(true, 32, 67.3); //using struct template for tuples
    print!("{}", mytuple.0);
    print!("{}", mytuple.1);
    println!("{}", mytuple.2);

    //now we can use struct as
    let student1 = Student {
        is_student: true,
        height: 110,
        class: 6,
    };
    print!("{}", student1.height); //to get value out of the struct
    print!("{}", student1.is_student);
    println!("{}", student1.class);

    let student_tuple: (bool, i32, char) = (true, 69, 'a'); //creating single tupple
    println!("{}", student_tuple.2);

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

    // Derefrencing

    let mut var1 = 36;
    let reference_var = &mut var1;
    *reference_var = 64;
    // let var2 = *reference_var;

    println!("{}", var1);
    // println!("{}",var2);
}
