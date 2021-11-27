fn main() {
    {
        let s1 = String::from("hello");
        let s2 = s1;

        println!("{}", s2);
    }

    {
        let s1 = String::from("hello");
        let s2 = s1.clone();

        println!("s1 = {}, s2 = {}", s1, s2);
    }

    {
        let s = String::from("hello");

        takes_ownership(s);

        let x = 5;

        makes_copy(x);
    }

    {
        let s1 = gives_ownership();

        let s2 = String::from("hello");

        let s3 = takes_and_gives_back(s2);

        println!("{} {}", s1, s3);
    }

    {
        let s1 = String::from("hello");

        let len = calculate_length(&s1);

        println!("The length of '{}' is {}.", s1, len);
    }

    {
        let mut s = String::from("hello");

        change(&mut s);

        println!("{}", s);
    }

    {
        let mut s = String::from("hello");

        let r1 = &s;
        let r2 = &s;
        println!("{} and {}", r1, r2);

        let r3 = &mut s;
        println!("{}", r3);
    }

    {
        let s = String::from("hello world");

        let hello = &s[0..5];
        let world = &s[6..11];

        println!("{} {}", hello, world)
    }
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
