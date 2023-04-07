// traits7.rs
//
// Your task is to replace the '??' sections so the code compiles.
// Don't change any line other than the marked one.
// Execute `rustlings hint traits7` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

#[allow(unused_imports)]
use std::fmt::{Debug, Formatter};

/// Struct representing a house
// TODO: Complete the Code
#[derive(??, Default)]
struct House {
    area_sqft: u32,
    purchase_date: String,
}

/// Struct representing a vehicle
// TODO: Complete the Code
#[derive(??, Default)]
struct Vehicle {
    name: String,
    model: String,
    purchase_date: String,
}

// TODO: Complete the code
trait Details: ?? {
    fn summary(&self) -> String;
}

impl Details for House {
    fn summary(&self) -> String {
        format!(
            "The house has an area of {} sqft and was purchased on {}",
            self.area_sqft, self.purchase_date
        )
    }
}
impl Details for Vehicle {
    fn summary(&self) -> String {
        format!(
            "The {} vehicle with model {} was purchased on {}",
            self.name, self.model, self.purchase_date
        )
    }
}

// TODO: Complete the code
fn foo(flag: bool) -> ?? {
    if flag {
        Box::new(House {
            area_sqft: 5000,
            purchase_date: "21 Nov 2017".to_string(),
        })
    } else {
        Box::new(Vehicle {
            name: "BMW".to_string(),
            model: "320d".to_string(),
            purchase_date: "13 Aug 2022".to_string(),
        })
    }
}

pub fn main() {
    let x = foo(true);
    println!("{:?}", x);
    // TODO: Complete the code
    // print the summary of the struct returned from the function `foo`
    println!("{}", ??);
}
