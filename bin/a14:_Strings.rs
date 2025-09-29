// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    name: String,
    age: i32,
    favourite_colour: String,
}

fn display_person(name: &str, colour: &str) {
    println!("{:?}'s favourite colour is {:?}", name, colour)
}

fn main() {
    let group = vec![
        Person {
            name: String::from("Adam"),
            age: 22,
            favourite_colour: String::from("Brown"),
        },
        Person {
            name: String::from("Billy"),
            age: 41,
            favourite_colour: String::from("Blue"),
        },
        Person {
            name: String::from("Clara"),
            age: 25,
            favourite_colour: String::from("Yellow"),
        },
        Person {
            name: String::from("Bobby"),
            age: 8,
            favourite_colour: String::from("Red"),
        },
        
    ];

    for person in group {
        if person.age <= 10 {
            display_person(&person.name, &person.favourite_colour);
        }
    }
}
