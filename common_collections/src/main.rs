fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    println!("v={:?}", v);
    let v = vec![1, 2, 3];

    let second: &i32 = &v[1];
    println!("second={second}");

    match v.get(2) {
        Some(third) => println!("third={third}"),
        None => println!("no element"),
    }

    match v.get(4) {
        Some(fifth) => println!("fifth={fifth}"),
        None => println!("no element"),
    }

    for i in &v {
        println!("{}", i);
    }

    let s = "initial contents".to_string();
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("s={s}");
    let s3 = s + &String::from("boy");
    println!("s3={s3}");
    // can't refer s here. It has been moved.

    use std::collections::HashMap;

    let mut scores = HashMap::new();
    let value = String::from("string value");
    scores.insert(String::from("blue"), value);

    let score = scores.get(&String::from("blue"));
    match score {
        Some(value) => println!("{}", value),
        None => println!("no value"),
    }
}
