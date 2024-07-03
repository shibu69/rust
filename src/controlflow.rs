
trait Printable {
    fn print(&self);
}
struct Sequence<T> {
    first: T,
    second: T,
    third: T,
}
#[derive(Debug,Default,Clone,PartialEq, Eq)]
struct Point {
    x:i32,
    y:i32
}

impl Printable for Point {
   fn print(&self) {
       println!("The x point is {} and y point is {}",self.x,self.y);
   }
}

impl<T> Sequence<T> {
    pub fn new(first: T, second: T, third: T) -> Self {
        Self {
            first,
            second,
            third,
        }
    }
}

pub fn main() {
    //Conditionals
    let age = 19;

    if age >= 18 {
        println!("Eligible for Driving");
    } else {
        println!("Not eligible for driving")
    }

    //Looping System
    let mut i = 5;
    loop {
        if i == 1 {
            println!("This is normal Loop of rust ");
        }
        if i == 0 {
            break;
        }
        print!("This is normal Loop of rust ");

        i -= 1;
    }

    let mut j = 5;
    while j > 0 {
        if j == 1 {
            println!("this is while loop in rust ")
        }
        print!("this is while loop in rust ");
        j -= 1;
    }

    for i in 1..10 {
        print!("{i}");
    }

    //Pattern Matching
    let n = 88;

    match n {
        1 => println!("It was one!"),
        2 => println!("It was two!"),
        3 | 4 => println!("It was a bit more than two!"),
        high if high >= 5 => println!("It was a high number!"),
        other => println!("It was {other}!"),
    }

    let marks = 30;

    match marks {
        0..=30 => println!("Failed with honour"),
        31..=60 => println!("Decent but not good"),
        61..=90 => println!("You are average, don't like you"),
        91..=100 => println!("You are my brilliant student"),
        _ => println!("You are something else"),
    }

    //ternary if is not present in rust in place of that we use

    let a = 0;

    let b = if a == 0 { 1 } else { 0 };

    println!("{}", b);

    //or

    let brownies = 1;
    let quantity = match brownies {
        0 => "no brownies eaten",
        1 => "One brownie eaten",
        _ => "too much brownies eaten",
    };

    println!("{quantity}");

    //generics

    let sequence_i32 = Sequence::new(1, 2, 3);

    let sequence_string = Sequence::new(
        String::from("first"),
        String::from("second"),
        String::from("third"),
    );

    println!(
        "sequence_i32: {}, {}, {}",
        sequence_i32.first, sequence_i32.second, sequence_i32.third
    );
    println!(
        "sequence_string: {}, {}, {}",
        sequence_string.first, sequence_string.second, sequence_string.third
    );

    //traits
    let value = "Hello";
    println!("regular: {}", value);
    println!("padded : {:-<7}", value);

    let points = Point{
        x:39,
        y:65
    };

    points.print();

    //Derive Debug

    let point2 = Point{
        x:3,
        y:60
    };

    println!("{:?}",point2);

    //Devlarative Macros

    macro_rules! create_vec {
        ( $( $item:expr ),* ) => {
            {
                let mut result = Vec::new();
                $(
                    result.push($item);
                )*
                result
            }
        }
    }
    let items = create_vec!(1, 2, 3);
    println!("{items:?}");

    //Procedular Macros
    
    // Custom Derive Macros

    let point3 = Point::default();
    let point4 = point3.clone();
    assert_eq!(&point3,&point4);
    assert!(point2.x.is_positive()); //for testing purposes

    println!("{:#?}",point3);

    //borrowing and referencing

    //pass by value
    let str = String::from("Jai Ho");
    let (str,length) = find_length(str);

    println!("The length of {str} is {length}"); 

    //pass by reference
       
    
}


fn find_length(value:String) -> (String,usize){
    let len = value.len();
    (value,len)
}
