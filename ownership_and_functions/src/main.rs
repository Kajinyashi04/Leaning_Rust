fn main() {
    let s = String::from("Hello");

    takes_ownerships(s);

    let x = 5;
    make_copy(x);
}

fn takes_ownerships(some_string: String) {
    println!("{some_string}");

}

fn make_copy(some_integer: i32) {
    println!("{some_integer}");
}