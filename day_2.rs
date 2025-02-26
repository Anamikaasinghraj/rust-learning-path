//mutable variables: values can be changed after assignment. Declared using let mut.

let mut a: u8 = 5; //mutable
let mut b: u8 =10; //mutable
let mut c: u8; //mutable

c = a;  // Store the value of 'a' in 'c'
a = b; // Assign the value of 'b' to 'a'
b = c; // Assign the original value of 'a' (stored in 'c') to 'b'

println!("The value of a is {} and b is {}", a, b);


//Immutale variables: values cannot be changed once assigned. Declared using let.

fn main() {
    let num: u8 = 12; //immutable
    println!("This is num {}", num);
}
