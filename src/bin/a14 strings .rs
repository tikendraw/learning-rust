
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

// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
struct Person {
    age: i32,
    name: String,
    color: String,
}

impl Person {
    // fn new(age: i32, name: String, color: String) -> Person {
    //     Person {
    //         age,
    //         name,
    //         color
    //     }
    // }


    fn print_info(&self) {
        println!("Name: {}", self.name);
    }

    fn print_color(&self) {
        println!("Favourite Color: {}", self.color);
    }

    fn print_age(&self) {
        println!("Age: {}", self.age);
    }

    fn print_all(&self) {
        self.print_info();
        self.print_color();
        self.print_age();
    }
}


fn main() {

    // * Create and store at least 3 people in a vector
    let people = vec![
        Person{age: 22, name: "ramesh".to_owned(),   color: "red".to_owned()},
        Person{age: 8,  name: "ram".to_owned(),      color: "saffron".to_owned()},
        Person{age: 10, name: "ramanand".to_owned(), color: "blue".to_owned()},
    ];

    // * Iterate through the vector using a for..in loop
    for person in &people {
        if person.age <= 10{
            person.print_all();
        }
    }
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function
}