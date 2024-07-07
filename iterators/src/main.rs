use std::{thread, time::Duration};

fn main() {
    let store = Inventory {
        shirts: vec![
            ShirtColor::Blue,
            ShirtColor::Blue,
            ShirtColor::Blue,
            ShirtColor::Red,
            ShirtColor::Red,
        ],
    };

    let user1 = Some(ShirtColor::Red);
    let give1 = store.giveaway(user1);

    println!("{:?}", give1);

    let user2 = None;
    let give2 = store.giveaway(user2);
    println!("{:?}", give2);

    //Closure in variables
    let expensive_closure = |num: u32| -> u32 {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let one = expensive_closure(1);
    println!("{one}");

    //Anothr closure example
    let outer_val = 56;
    let closure_annotated = |i: i32| -> i32 { i + outer_val };
    //let closure_infered = |i| i + outer_val;

    //Capturing value by reference
    let color = String::from("Green");

    let print = || println!("{color}");
    print();
}

#[derive(Debug)]
enum ShirtColor {
    Blue,
    Red,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, preference: Option<ShirtColor>) -> ShirtColor {
        preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Blue => num_blue += 1,
                ShirtColor::Red => num_red += 1,
            }
        }

        if num_red > num_blue {
            return ShirtColor::Red;
        }

        ShirtColor::Blue
    }
}
