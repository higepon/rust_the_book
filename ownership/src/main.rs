fn print_string(s: String) {
    println!("string is {s}");
}

fn hoge(s: String) -> String {
    return s;
}

fn calc_len(s: &String) -> usize {
    s.len()
}

fn concat_hello(s: &mut String) {
    s.push_str(", hello");
}

fn main() {
    {
        let mut s = String::from("Hello");
        println!("s={s}");

        s.push_str(", world");
        println!("s={s}");
    }
    // s is returned

    {
        let s1 = String::from("hello");
        let s2 = s1; // This actually moves!
        println!("s2={s2}");

        // s1 is no longer valid
        // println!("s1={s1}");
    }

    {
        let s1 = String::from("hello");
        let s2 = s1.clone(); // deep copy
        println!("s2={s2}");

        println!("s1={s1}");
    }

    {
        let x = 5;
        let y = x;
        println!("x={x} y={y}");
    }

    {
        let s = String::from("Hello");
        print_string(s);
        // println!("s={s}"); // invalid.
    }

    {
        let s1 = String::from("arg");
        println!("s1={s1}");
        let s2 = hoge(s1);
        // println!("s1={s1}"); invalid
        println!("s2={s2}");
    }
    {
        let s1 = String::from("arg");
        let len = calc_len(&s1);
        println!("s1={s1} len={len}");
    }
    {
        let mut s1 = String::from("I'm higepon");
        concat_hello(&mut s1);
        println!("s1={s1}");
    }
}
