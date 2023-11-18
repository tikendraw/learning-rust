//exercise

//*print 10,20, thirty , 40 in a for loop */
//*print total number of elements */

//*NOTES */
//*Use vector to stor that */
// use for loop
// decide wheather to print thirty inside the loop
// use len to print element

fn main(){
    let vecto: Vec<i32> = vec![10,20,30, 40];

    for i in &vecto {
        match i {
            30 => println!("Thirty"),
            _  => println!("{}", i)
        }
    }
    println!("Total elements: {a}", a=&vecto.len());
}