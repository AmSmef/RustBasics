// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

fn main() {

    let firstname = "Adam";
    let surname = "Smith";

    display_full_name(firstname, surname);
}

fn display_first_name (firstname: &str) {
    println!("{firstname}")
}

fn display_surname (surname: &str) {
    println!("{surname}")
}

fn display_full_name (firstname: &str, surname: &str) {
    display_first_name(firstname);
    display_surname(surname);
}
