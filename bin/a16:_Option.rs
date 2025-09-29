// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student {
    name: String,
    locker: Option<i32>,
}

fn main() {

    let class_clown = Student {
        name: "Adam".to_owned(),
        locker: Some(47),
    };

    println!("The class clown is {:?}", class_clown.name);

    match class_clown.locker {
        Some(num) => println!("The class clown has a bomb in locker {:?}", num),
        None => println!("The class clown has no bomb... I swear"),
    }
}
