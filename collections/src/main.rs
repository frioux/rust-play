use std::collections::HashMap;

fn main() {
    // vectors
    let v: Vec<i32> = Vec::new();
    println!("v: {:?}", v);

    let mut v = vec![1, 2, 3];
    println!("v: {:?}", v);
    v.push(4);
    println!("v: {:?}", v);
    println!("v popped: {}", v.pop().unwrap());
    println!("v: {:?}", v);

    let x = v[1];
    v.push(1);
    println!("v[1]: {}", x);
    println!("v.get(2): {:?}", v.get(9));

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }


    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("v: {:?}", v);

    #[derive(Debug)]
    enum Log {
        Access(String, u32),
        Error(String),
    }

    let log = vec![
        Log::Access(String::from("/cool"), 3),
        Log::Error(String::from("oh noes")),
    ];
    println!("log: {:?}", log);

    // strings
    let mut s = String::new(); 
    s.push('💀');
    println!("s: {}", s);

    let s = "initial contents".to_string();
    println!("s: {}", s);

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("s: {}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("s2: {}, s3: {}", s2, s3);


    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{}", s);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    // hashmaps
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
   
    println!("{:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("{:?}", scores);

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    println!("{}", scores.get(&team_name).unwrap());

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    // exercises
    ex1(vec![1, 6, 7, 2, 3, 4, 2]);
    ex2(&String::from("hello friend"));
    ex3();
}

fn ex1(mut nums: Vec<i32>) {
   let mut sum: f32 = 0.0;
   for i in &nums {
      sum += *i as f32;
   }
   println!("average: {}", sum/(nums.len() as f32));

   nums.sort_unstable();
   println!("median: {}", nums[nums.len() / 2]);
   // hist counts entries in nums [1,1,2,3] -> {1: 2, 2: 1, 3: 1}
   let mut hist = HashMap::new();
   for i in nums {
       let e = hist.entry(i).or_insert(0);
       *e += 1;
   }

   // inv_hist inverts hist {1: 2, 2: 1, 3: 1} -> {1: [2, 3], 2: 1}
   let mut inv_hist = HashMap::new();
   for (k, v) in &hist {
       let e = inv_hist.entry(v).or_insert_with(Vec::<i32>::new);
       e.push(*k);
   }
   let mut max = 0;
   for k in inv_hist.keys() {
      if **k > max {
          max = **k
      }
   }
   println!("mode: {:?}", inv_hist.get(&max).unwrap());
}

fn ex2(eg: &str) {
   let words: Vec<String> = eg.split_whitespace()
       .map(String::from)
       .collect();

   let mut pigwords: Vec<String> = Vec::with_capacity(words.len());

   for w in &words {
       let end = w.get(1..).unwrap();
       let c = w.get(0..1).unwrap();
       match c {
         "a"|"e"|"i"|"o"|"u" => pigwords.push(format!("{}{}-hay", &String::from(c), &String::from(end))),
         _ => pigwords.push(format!("{}-{}ay", &String::from(end), &String::from(c))),
       }
   }
   println!("pigwords: {:?}", pigwords);
}

use std::io;
use regex::Regex;

fn ex3() {
   let mut departments: HashMap<String, Vec<String>> = HashMap::new();

   let re = Regex::new(r"(?x)
   (?P<quit>q(uit)?) |
   (?P<add>add)\s+(?P<person>\S+)\s+to\s+(?P<dept>\S+) |
   (?P<show>show)(?:\s+(?P<showdept>\S+))?
   ").unwrap();

   loop {
        println!("Please input your command (quit, show [$dept], add <$person> to <$dept>");
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("couldn't read line");

        match re.captures(&command) {
            None => continue,
            Some(c) => {
                if c.name("quit").is_some() {
                    break
                }
                if c.name("show").is_some() {
                    let showdept = c.name("showdept");
                    if showdept.is_some() {
                        match departments.get(&String::from(showdept.unwrap().as_str())) {
                            None => continue,
                            Some(dept) => {
                                let mut dept = dept.clone();
                                dept.sort();
                                for person in dept {
                                    println!("{}", person);
                                }
                            }
                        }
                    } else {
                        println!("{:?}", departments);
                    }
                }
                if c.name("add").is_some() {
                    let person = String::from(c.name("person").unwrap().as_str());
                    let dept = String::from(c.name("dept").unwrap().as_str());

                    let e = departments.entry(dept).or_insert_with(Vec::<String>::new);
                    e.push(person);
                }
            }
        }
   }
}
