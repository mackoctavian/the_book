fn main() {
    let integer = Point { x: 5, y: 4 };
    let float = Point { x: 5.0, y: 4.0 };
}

//Function largest to find largest number in list
fn largest(list: &[i32]) -> &i32 {
    let mut largest_number = &list[0];

    for item in list {
        if item > largest_number {
            largest_number = item;
        }
    }

    return &largest_number;
}

//Function largest using generics (won't compile)
fn another_largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

//Generics in struct
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

//Generics in enum
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
