use std::fs::read_to_string;

fn main() {
    let ans = fib(10);
    println!("{}", ans);

    // string
    let name = String::from("Aniket");
    let len = get_str_len(name);
    println!("Length of the string is {}", len);

    // struct
    let user1 = User {
        first_name: String::from("Aniket"),
        last_name: String::from("Paul"),
        age: 22
    };
    println!("User: {} {} is {} years old", user1.first_name, user1.last_name, user1.age);

    // struct implementation
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };
    println!("Area of the rectangle is {}", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&Rectangle { width: 10, height: 40 }));
    println!("Debug: {}", Rectangle::debug());

    // enum
    let my_direction = Direction::North;

    // enum with asscociated values
    let rect = Shape::Rectangle(10.0, 20.0);
    println!("{}", calculate_area(rect));

    // Option/Result Enum
    // Option
    let idx = find_first_a(String::from("Aniketa"));
    match idx {
        Some(val) => println!("Index of first 'a' is {}", val),
        None => println!("No 'a' found")
    }
    // Result
    let result = read_to_string("a.txt");
    match result {
        Ok(content) => println!("{}", content),
        Err(err) => println!("Error: {}", err)
    }
}

// if-else, function, loops
fn fib(num: i32) -> i32 {
    let mut first = 0;
    let mut second = 1;
    if num==0 {
        return first;
    } else if num==1 {
        return second;
    } else {
        for _ in 0..num-1 {
            let temp = second;
            second = second+first;
            first = temp;
        }
        return second;
    }
}

// String
fn get_str_len(str: String) -> usize {
    str.chars().count() // implicit return (without return statement & ;)
}

// Struct
struct User {
    first_name: String,
    last_name: String,
    age: i32
}
struct Rectangle {
    width: u32,
    height: u32
}
impl Rectangle { // struct implementation
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn debug() -> i32 { // similar to static method
        return 1;
    }
}

// Enum
enum Direction {
    North,
    East,
    South,
    West
}
enum Shape {
    Rectangle(f64, f64), // Rectangle(width, height)
    Circle(f64) // Circle(radius)
}

// Pattern Matching
fn calculate_area(shape: Shape) -> f64 {
    let area = match shape {
        Shape::Rectangle(width, height) => width * height,
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius
    };
    return area;
}

// Option/Result Enum (Inbuilt enums provided by rust to handle null values (Option) and errors(Result))
// Option enum lets you return Some value or None value
// Rust doesn't have null values, instead it has options
// pub enum Option<T> {
//     None,
//     Some(T),
// }
fn find_first_a(s: String) -> Option<i32> {
    for(index, char) in s.chars().enumerate() {
        if char == 'a' {
            return Some(index as i32);
        }
    }
    return None;
}

// Result enum lets you return either Ok value or Err value (used for error handling)
