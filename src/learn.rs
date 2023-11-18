#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

// fn main() {
//     println!("What is Your Name?");
//     let mut name: String = String::new();
//     let greeting: &str = "Nice to meet You";
//     io::stdin().read_line(&mut name).expect("Didn't Receive Input");
//     println!("Hello, {}! {}", name.trim_end(), greeting);

// }


// fn main() {
//     let mut num: String = String::new();
//     println!("Enter a Number: ");
//     io::stdin().read_line(&mut num).expect("Must be a number");
//     let num: i32 = num.trim().parse().expect("Please enter a valid number");

//     println!("You entered: {}", num);

//     if num >= 200 {
//         println!("Number is greater than or equal to 200");
//     } else if 50 < num && num < 200 {
//         println!("Number is between 50 and 200 (exclusive)");
//     } else {
//         println!("Number is less than or equal to 50");
//     }
// }


// Loop
// fn main(){
//     let mut a: i32 = 0;
//     let mut b: String = "Odd".to_string();
//     loop{
//         if a == 15{
//             break;
//         }

//         if a % 2 == 0 {
//             b = "Even".to_string();
//         }else{
//             b = "Odd".to_string();
//         }

//         println!("a: {a:?} is {b}");
//         a += 1;
//     }
// }

// While loop

// fn main(){
//     let mut a: i16 = 0;

//     while a < 10 {
//         println!("a: {a:?}");
//         a += 1;
    
//     }
// }


// // add function
// fn add_fn(a:i32 , b:i32) -> i32 {
//     a+b
// }

// fn main() {
//     let mut num1: String = String::new();
//     let mut num2: String = String::new();
//     println!("Enter a number:");
//     io::stdin().read_line(&mut num1).expect("Must be a Integer");
//     println!("Enter Another Number: ");
//     io::stdin().read_line(&mut num2).expect("Must be a Integer");
    
//     let num1 = num1.trim().parse().expect("Must be Integer");
//     let num2 = num2.trim().parse().expect("Must be a integer");

//     let sum = add_fn(num1, num2);
//     println!("Sum: {}", sum);
// }



// // Ownership

// struct Grocery{
//     name: String,
//     stock: i16,
//     price: i16
// }


// fn display_name(name: &String){
//     println!("Name: {}", name);
// }

// fn display_price_and_stock(price:&i16, stock:&i16){
//     println!("Price: {price} Stock: {stock}");
// }
// fn main(){
//     let parleg = Grocery{
//         name: "Parleg".to_string(),
//         stock: 10,
//         price: 5
//     };
//     display_name(&parleg.name); // putting & before variables borrows it and doen't get deleted from memory, else it does. also change the arguments in the function 
//     display_price_and_stock(&parleg.price, &parleg.stock);
//     display_name(&parleg.name);
//     // println!("parleg: {parleg}");
    
// }


// Ownership and Impl

// struct Grocery{
//     name: String,
//     stock: i16,
//     price: i16
// }

// impl Grocery{
//     fn new() -> Self{ // this will create instance of object big Self (like classmethod in python) ,
//         Self{
//             name: "random_grocery".to_string(),
//             stock: 1,
//             price: 5
//         }
//     }

//     fn display_name(&self){ // small self reference to the object and borrows it ,so it doesn't get dropped from memory
//         println!("Name: {}", self.name);
//     }

//     fn display_price_and_stock(&self){
//         println!("Price: {a} \nStock: {b}", a=self.price, b=self.stock);
//     }
// }

// fn main(){
//     let parleg = Grocery{
//         name: "Parleg".to_string(),
//         stock: 10,
//         price: 5
//     };
//     parleg.display_name();
//     parleg.display_price_and_stock();
//     // println!("parleg: {parleg}");

//     println!("Another parleg with new(it's like a class method from python)");
//     let pp = Grocery::new();

//     pp.display_name();
//     pp.display_price_and_stock();
    
// }


// // exercise

// enum BoxColor{
//     Red,
//     Blue,
//     Green
// }

// impl BoxColor{
//     fn print_color(&self){
//         match self{
//             BoxColor::Red => println!("Red"),
//             BoxColor::Blue => println!("Blue"),
//             BoxColor::Green => println!("Green")
//         }
//     }
// }

// struct Dimensions{
//     height: f32,
//     length: f32,
//     width: f32
// }

// impl Dimensions {
//     fn print_dimensions(&self){
//         println!("Height: {a}", a=self.height);
//         println!("Length: {a}", a=self.length);
//         println!("Width: {a}", a=self.width);
//     }
    
// }
// struct ShippingBox{
//     weight:f32,
//     dimensions: Dimensions,
//     color: BoxColor
// }

// impl ShippingBox{
//     fn new(weight: f32, dimensions: Dimensions, color: BoxColor) -> Self{
//         Self{
//             weight,
//             dimensions,
//             color
//         }
//     }

//     fn print_box_details(&self){
//         println!("Weight: {a}", a=self.weight);
//         self.dimensions.print_dimensions();
//         self.color.print_color();
//     }
// }

// fn main(){
//     let box1 = ShippingBox::new(10.0, Dimensions{height:10.0, length:10.0, width:10.0}, BoxColor::Red);
//     box1.print_box_details();
// }
