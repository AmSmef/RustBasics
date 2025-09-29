// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct Grocery {
    quantity: i32,
    id: i32,
}

fn display_quantity(grocery: &Grocery) {
    println!("Quantity: {:?}", grocery.quantity)
}

fn display_id(grocery: &Grocery) {
    println!("ID Number: {:?}", grocery.id)
}

fn main() {

    let banana = Grocery {
        quantity: 16,
        id: 47,
    };

    display_id(&banana);
    display_quantity(&banana);

}
