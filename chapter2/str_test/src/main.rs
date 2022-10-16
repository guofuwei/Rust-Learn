fn main() {
    let x = Some(String::from("a"));
    let y = true;
    match &x {
        Some(s) if y => println!("1:{}", s),
        Some(s) => println!("2:{}", s),
        _ => println!("jjj"),
    }
    println!("{:?}", x);
}
