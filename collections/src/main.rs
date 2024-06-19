use std::{collections::HashMap, mem};

fn main() {
    println!("Hello, world!");
}

//Learning about vector
fn using_vec() {
    let mut v =vec![1, 5];
    let v2 = Vec::from([1,2,3,4]);
    let v3 = vec![0; 5];
    v.push(2);

    //Reading the value of vectors
    // 1. Method One: Using Indexing
    let third = &v[2];
    println!("The third element is {third}");

    //2. Method Two: using get()
    let second = v.get(1);
    match second {
        Some(second) => println!("The second element is {second}"),
        None => println!("There is no second element")
    }

    //Iterating over
    for i in &mut v {
        *i += 1;
        println!("{i}");
    }

    //Popping
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{top}");
    }
}

fn store_multiple_types() {
    enum SpreadSheet {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let row = vec![
        SpreadSheet::Int(3),
        SpreadSheet::Float(5.0),
        SpreadSheet::Text(String::from("Blue")),
    ];
}

fn using_strings() {
    let mut s = String::from("foo");
    s.push_str(" bar");

    //Iterating over a string
    for i in s.chars() {
        println!("{i}");
    }
}

fn learning_hashmap() {
    let mut books = HashMap::new();

    books.insert("The Rust Programming Language".to_string(), "My fav book");
    books.insert("Rust By Example".to_string(), "Masterpiece");
    books.insert("Rust By Example 2".to_string(), "Masterpiece");

    //Checking for specific one
    if !books.contains_key("misserable") {
        println!("We have got {} but none looks miserable", books.len());
    }

    //Removing a book
    books.remove("Rust By Example 2");

    //Look up the values associated with some keys
    let to_find = ["Rust By Example", "The Rust Programming Language"];

    for &book in &to_find {
        if let Some(boo) = books.get(book) {
            println!("{boo}");
        }
    }

    //Iterate over everything
    for (book, review) in &books {
        println!("{book}, {review}");
    }
}
