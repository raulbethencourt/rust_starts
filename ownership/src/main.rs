fn main() {
    fn first() {
        let x: &str = "hello ";
        second(x);
    }

    fn second(n: &str) {
        let mut x: String = String::from("world");
        x.push_str(" and universe");
        x.insert_str(0, n);
        let t = x.clone();
        let len = calculate_length(&t);
        let x_len = calculate_length(&mut x);
        println!("{} / {} / {} / {}", x, t, len, x_len);
    }

    {
        let s = "yepaaa !!!";
        let n = s;
        println!("{} / {}", s, n);
    }

    fn calculate_length(str: &String) -> usize {
        str.len()
    }

    first();
    // Memory and Allocation
}
