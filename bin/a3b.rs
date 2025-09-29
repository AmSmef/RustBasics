// Topic: Flow control using if..else if..else
//
// Program requirements:
// * Display ">5", "<5", or "=5" based on the value of a variable
//   is > 5, < 5, or == 5, respectively
//
// Notes:
// * Use a variable set to any integer value
// * Use an if..else if..else block to determine which message to display
// * Use the println macro to display messages to the terminal

fn main() {

    let age = 7;

    if age > 5 {
        println!("You are older than 5, congrats!")
    } else if age < 5 {
        println!("You are the youngest prson ever!!!")
    } else {
        println!("AHHH, you must be 5")
    }

}
