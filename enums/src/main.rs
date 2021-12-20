fn main() {
    println!("{}", m(&String::from("")));
}

fn m(s: &str) -> &str {
    match s {
        "" => "empty",
        "herp" => "dong",
        // x => x,
    }
}
