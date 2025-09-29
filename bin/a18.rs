// Topic: Result
//
// Requirements:
// * Create an structure named `Adult` that represents a person aged 21 or older:
//   * The structure must contain the person's name and age
//   * Implement Debug print functionality using `derive`
// * Implement a `new` function for the `Adult` structure that returns a Result:
//   * The Ok variant should contain the initialized structure, but only
//     if the person is aged 21 or older
//   * The Err variant should contain a String (or &str) that explains why
//     the structure could not be created
// * Instantiate two `Adult` structures:
//   * One should be aged under 21
//   * One should be 21 or over
// * Use `match` to print out a message for each `Adult`:
//   * For the Ok variant, print any message you want
//   * For the Err variant, print out the error message

#[derive(Debug)]
struct Adult {
    name: String,
    age: i32,
}

impl Adult {
    fn new (name: &str, age: i32) -> Result<Self, &str> {
        if age >= 21 {
            Ok(Self {
                age,
                name: name.to_string(),
            })
        } else {
            Err("Age must be at least 21")
        }
    } 
}

fn main() {

    let husband = Adult::new("Bruce", 54);

    let wife = Adult::new("Margaret", 49);

    let child = Adult::new("Bobby", 7);

    match husband {
        Ok(husband) => println!("{:?} is {:?} years old", husband.name, husband.age),
        Err(e) => println!("Error message: {:?}", e)
    }

    match wife {
        Ok(wife) => println!("{:?} is {:?} years old", wife.name, wife.age),
        Err(e) => println!("Error message: {:?}", e)
    }

    match child {
        Ok(child) => println!("{:?} is {:?} years old", child.name, child.age),
        Err(e) => println!("Error message: {:?}", e)
    }

}
