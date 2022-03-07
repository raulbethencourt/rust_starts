fn main() {
    fn first() {
        let x: &str = "hello ";
        second(x);
    }

    fn second(n: &str) {
        let mut x: String = String::from("world");
        x.push_str(" and universe");
        x.insert_str(0, n);
        println!("{}", x);
    }

    {
        let s = "yepaaa !!!";
        println!("{}", s);
    }

    first();
    // Memory and Allocation
}
