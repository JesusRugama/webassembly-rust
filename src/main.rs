fn add(num_one: i16, num_two: i16) -> i16 {
    let a = num_one + num_two;

    a
}

fn main() {
    let mut total = add(23, 56);
    let mut free_shipping = false;

    if total <= 12 {
        println!("lower");
    } else {
        println!("higher");
    }

    total = match free_shipping {
        true => total + 10,
        false => total + 1,
    };

    match total {
        80 => println!("eighty"),
        90 => println!("ninety"),
        _ => println!("something else"),
    }

    println!("Total: {:?}", total);
}