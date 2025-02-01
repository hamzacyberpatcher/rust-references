fn main() {
    println!("Hello, world!");

    let mut s = String::from("hello");

    meow(&mut s);

    println!("{s}");

    let mut x: String = String::from("neko");

    let r1: &String = &x;
    let r2: &String = &x;

    println!("{}, {}", r1, r2);

    println!("{}", r1);

    let r3: &mut String = &mut x;

    r3.push_str(" order");

    println!("{r3}");

}

fn meow(s: &mut String) {
    s.push_str(" meow");
}
