fn print_numbers(x: [i32; 5]) {
    print!("[");
    for (index, item) in x.iter().enumerate() {
        if index == x.len() - 1 {
            print!("{}", item);
        }
        else {
            print!("{}, ", item);
        }
    }
    println!("]");
}

fn print_string(x: &String) {
    println!("{}", x);
}

fn main() {
    let s = String::from("Hello, World!");
    print_string(&s);

    let x: [i32; 5] = [10, 20, 30, 40, 50];
    print_numbers(x);
}
