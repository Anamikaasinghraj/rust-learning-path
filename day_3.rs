fn main() {
    //&str - fixed length strings 

    let string_literal = "Hii, orientalite!!!";
    println!("This is string literal {}", string_literal);

}

fn main() {
//String - dynamic length strings

let mut string_literal:String = String::from("Hii, orientalite!!!");
string_literal.push_str(" What's up?");
println!("Thi is string literal {}",string_literal);
}