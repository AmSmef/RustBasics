// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

struct Box {
    dimensions: Dimensions,
    weight: f64,
    colour: Colour,
}

struct Dimensions {
    height: i32,
    width: i32,
    depth: i32,
}

enum Colour {
    Blue,
    Red,
    Brown,
}

impl Colour {
    fn print(&self) {
        match self {
            Colour::Blue => println!("Colour: Blue"),
            Colour::Red => println!("Colour: Red"),
            Colour::Brown => println!("Colour: Brown"),
        };
    }
}

impl Dimensions {
    fn print(&self) {
        println!("Dimensions: {:?}", (self.height * self.width * self.depth))
    }
}

impl Box {
    fn new(weight: f64, dimensions: Dimensions, colour: Colour) -> Self {
        Self {
            dimensions,
            weight,
            colour,
        }
    }

    fn show_box(&self) {
        self.colour.print();
        self.dimensions.print();
        println!("Weight: {:?}", self.weight);
    }
}

fn main() {
    let small_dimensions = Dimensions {
        width: 2,
        height: 3,
        depth: 6,
    };

    let small_box = Box::new(45.5, small_dimensions, Colour::Red);
    small_box.show_box();

}
