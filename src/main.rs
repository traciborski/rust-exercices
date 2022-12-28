#![allow(unused_variables, dead_code, unused_mut, unused_imports)]

use std::collections::HashMap;

fn main() {
    let five = Box::new(5);
    let five2: i32 = 5;
}

// fn main() {
//     let mut s1 = String::new();
//     s1.push_str("Hello");
//     println!("s1: len = {}, capacity = {}", s1.len(), s1.capacity());

//     let mut s2 = String::with_capacity(s1.len() + 1);
//     s2.push_str(&s1);
//     s2.push('!');
//     println!("s2: len = {}, capacity = {}", s2.len(), s2.capacity());
// }

// fn main() {

//     let h = HashMap::<&str, i32>::new();

//     match std::env::args().next().as_deref() {
//         Some("cat") => println!("Will do cat things"),
//         Some("ls")  => println!("Will ls some files"),
//         Some("mv")  => println!("Let's move some files"),
//         Some("rm")  => println!("Uh, dangerous!"),
//         None        => println!("Hmm, no program name?"),
//         _           => println!("Unknown program name!"),
//     }
// }

// fn main() {
//     //let v = vec![10, 20, 30];
//     let mut v2 = [10, 20, 30];
//     let v3 = &v2[1..4];
//     for x in v3 {
//         println!("x: {x}");
//     }
// }