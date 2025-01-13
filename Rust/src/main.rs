use core::prelude::v1;
use std::{collections::HashMap, fs::read_to_string, string};

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



    // Collections

    // Vectors
    // let mut vec = Vec::new();
    // vec.push(1);
    // vec.push(2);
    // vec.push(3);
    // vec.push(4);
    let mut vec = vec![1,2,3,4]; // same as above
    even_filter(&mut vec);
    println!("{:?}", vec);

    // HashMap
    let mut users = HashMap::new();
    users.insert(String::from("Aniket"), 22);
    users.insert(String::from("Aniket"), 23); // overwrites the value
    let first_user_age = users.get("Aniket"); // Returns Option<>
    match first_user_age {
        Some(age) => println!("First user's age is {}", age),
        None => println!("No user found")
    }
    for (key, value) in users.iter() { // Iterator in hashmap
        println!("{}: {}", key, value);
    }

    // Iterators
    // iter() -> provides a way to iterate over the elements of a collection by borrowing them => you can't mutate the elements
    let mut v1 = vec![1,2,3,4,5,6,7,8,9,10];
    let v1_iter1 = v1.iter();
    for val in v1_iter1 { 
        println!("{}", val); 
    }
    // let mut v1_iter1 = v1.iter();
    // while let Some(val) = v1_iter1.next() {
    //     println!("{}", val);
    // }
    
    // iter_mut() -> mutable iterator
    let mut v1_iter2 = v1.iter_mut();
    for val in v1_iter2 { 
        *val += 1; 
    }
    println!("{:?}", v1);

    // into_iter() -> same as iter, except that it consumes the collection and takes ownership of the elements (has performance benefits)
    let mut v1_iter3 = v1.into_iter();
    for val in v1_iter3 {  // same if we directly use v1
        println!("{}", val); 
    }
    // println!("{:?}", v1); // will throw error as v1 is already consumed

    // Consuming adaptors
    let vec1 = vec![1,2,3];
    let vec1_iter = vec1.iter();
    let total: i32 = vec1_iter.sum();
    assert_eq!(total, 6);
    // let total_again = vec1_iter.sum(); // vec1_iter can't be used again

    // Iterator adaptors
    let vec123 = vec![1,2,3];
    let iter123_1 = vec123.iter();
    let iter123_2 = iter123_1.map(|x| x+1);
    // let iter123_3 = iter123_2.filter(|x| *x%2==0);
    for x in iter123_2 { 
        println!("{}", x); // prints 2 3 4
    }
    println!("{:?}", vec123); // prints 1 2 3

    // Q1. Filter all odd values of a vector then double each value & create a new vector
    let v_new = vec![1,2,3,4,5,6,7,8,9,10];
    let ans = filter_and_map(v_new);
    println!("{:?}", ans);


    // Strings
    let mut name = String::from("Aniket"); // creation
    name.push_str(" Paul"); // mutation
    println!("{}", name);
    name.replace_range(6..name.len(), ""); // deletion
    println!("{}", name);

    // let mut word = String::from("Hello world");
    // let word2  = &word[0..5]; // slicing
    // # At any given point, you can either have one mutable reference or any no. of immutable references
    // therefore, the below code will throw an error as clear() method takes mutable reference
    // word.clear();
    // println!("{}", word2);

    // Slices (memory safe)
    // Q2 Write a function that takes a string and returns the first word it finds in that string
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("{}", word);
    
    // Generics
    let bigger = largest(1, 2);
    let bigger_char = largest('a', 'b');
    println!("{}", bigger);
    println!("{}", bigger_char);

    // Trait
    let users = UserStruct {
        name: String::from("Aniket"),
        age: 22
    };
    println!("{}", users.summarize());
    notify(&users);
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


// Vector
fn even_filter(v: &mut Vec<i32>){
    let mut i = 0;
    while i<v.len() {
        if v[i]%2 != 0 {
            v.remove(i);
        } else {
            i+=1;
        }
    }
}

// Q1. 
fn filter_and_map(v: Vec<i32>) -> Vec<i32> {
    let new_iter = v.iter().filter(|x| *x%2==1).map({|x| x*2});
    let new_vec: Vec<i32> = new_iter.collect(); // collect() method converts iterator to vector
    return new_vec;
}

// Q2.
// Using string
// fn first_word(s: String) -> String {
//     let mut ans = String::from("");
//     for i in s.chars() {
//         if i == ' ' {
//             break;
//         }
//         ans.push_str(&i.to_string());
//     }
//     return ans;
// }
// Problem --> 
// 1. We are taking ownership of the string
// 2. We take up double the memory
// 3. If the `s` string gets cleared, the `word` string will still be there

// What we want is a `view` into the original string & not copy it over
// Using Slices
fn first_word(s: &String) -> &str {
    let mut space_index = 0;
    for i in s.chars() {
        if i == ' ' {
            break;
        }
        space_index += 1;
    }
    return &s[0..space_index];
}

// Generics
fn largest<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

// Trait
pub trait Summary {
    fn summarize(&self) -> String; // you can write the function implementation here as well, which will be used as the default implementation, if not implemented by the struct
}
pub trait Fix {
    fn fix(&self) -> String {
        String::from("Fixed")
    }
}
struct UserStruct {
    name: String,
    age: u32
}
impl Summary for UserStruct {
    fn summarize(&self) -> String {
        format!("Name: {}, Age: {}", self.name, self.age)
    }
}
impl Fix for UserStruct {}

// Traits as parameters
pub fn notify(item: &impl Summary) { // this function can take any type that implements Summary trait
    println!("Summary: {}", item.summarize());
}
// the above function internally converts into trait bounds, something like...
// pub fn notify<T: Summary + Fix>(item: &T)  ----> input should be implementing both Summary and Fix traits