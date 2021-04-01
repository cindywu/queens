// Tuples group together values of different types
// Max 12 elements

pub fn run() {
 let person: (&str, &str, i8) = ("Cindy", "Seattle", 32);

 println!("{} is from {} and she is {}.", person.0, person.1, person.2);
}