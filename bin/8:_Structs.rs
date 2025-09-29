// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavour {
    Cola,
    Cherry,
    Orange,
}

struct Drink {
    flavour: Flavour,
    oz: f64,
}

fn main() {

    let dr_pepper = Drink {
        flavour: Flavour::Cherry,
        oz: 25.5,
    };

    let lucozade = Drink {
        flavour: Flavour::Orange,
        oz: 18.5,
    };

    display_drink_info(dr_pepper);
    display_drink_info(lucozade);
}

fn display_drink_info(drink: Drink) {
    match drink.flavour {
        Flavour::Cola => println!("Flavour: Cola"),
        Flavour::Cherry => println!("Flavour: Cherry"),
        Flavour::Orange => println!("Flavour: Orange"),
    }

    println!("Fluid Ounces: {:?}", drink.oz)

}
