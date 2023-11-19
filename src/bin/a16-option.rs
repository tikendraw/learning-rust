// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student{
    name: String,
    locker: Option<i32>
}

fn main() {
    let students = vec![
        Student{name: "aarambh".to_owned(), locker: Some(12)},
        Student{name: "anant".to_owned(), locker: None}
    ];
    for student in students {
        match student.locker{
            Some(num) => println!("Name: {:?}\nLocker: {:?}", student.name, num),
            None => println!("Name: {:?}\nLocker: Locker not assigned", student.name)
        }
    }
}