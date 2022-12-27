fn main() {
    println!("{:?}", vec![String::from("foo"), String::from("bar")].into_iter().next());
    println!("{:?}", vec![String::from("foo"), String::from("bar")].iter().next());

    println!();

    let v: Vec<String> = vec![String::from("foo"), String::from("bar")];

    for word in &v {
        print_type_of(&word);
        println!("{word}");
    }

    println!();
    
    for word in v {
        print_type_of(&word);
        println!("{word}");
    }

    // for word in v.iter().into_iter().into_iter() {
    //     //println!("{word} {}", print_type_of(&word));
    //     print_type_of(&word);
    // }
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
