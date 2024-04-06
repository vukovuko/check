use std::io::stdin;

fn main() {
    let mut name = String::new();
    println!("What is your name?");
    stdin().read_line(&mut name).expect("We didn't get the name");
    if let Some('\n') = name.chars().next_back() {
        name.pop();
    }
    if name == "Vuko" {
        println!("Access Granted!");
    } else {
       println!("Permission Denied!"); 
    }
}
