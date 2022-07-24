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
}
