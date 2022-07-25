fn my_function() {
    println!("thank you for calling");
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    let x = 5;
    println!("x={x}");

    let mut y = 6;
    println!("y={y}");
    y = 7;
    println!("y={y}");

    const SEVEN: u32 = 7;
    println!("seven={SEVEN}");

    let z = 5;
    let z = z + 2;
    println!("z = {z}");
    {
        let z = z * 2;
        println!("z={z}");
    }
    println!("z={z}");

    let spaces = "    ";
    let spaces = spaces.len();
    println!("spaces={spaces}");

    let x = 2.3;
    println!("floating x = {x}");

    println!("{}, {}", 2 / 3, 2.0 / 3.0);

    let x = false;
    println!("x={x}");

    let c = 'c';
    println!("c={c}");

    let tup: (u32, f64, u8) = (500, 6.4, 3);
    let (x, y, z) = tup;
    println!("tup={x} {y} {z}");
    println!("{} {}", tup.0, tup.2);

    let a = [1, 2, 3, 4, 5];
    println!("a={}", a[4]);
    let a = [3; 100];
    println!("a={}", a[99]);

    my_function();

    let x = add(3, 4);
    println!("{}", x);
    if x < 5 {
        println!("x is smaller than 5");
    } else {
        println!("x is not smaller than 5");
    }

    let y = if x == 7 { true } else { false };
    println!("value is ={}", y);

    let mut counter = 0;
    let result = 'counting_up: loop {
        counter += 2;
        if counter == 100 {
            break 'counting_up counter * 2;
        }
    };
    println!("result = {result}");

    for e in a {
        println!("{e}");
    }
}
