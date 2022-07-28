struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct AlwaysEqual;

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rect: Rectangle) -> u32 {
    rect.width * rect.height
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    println!("Hello, world!");

    let user1 = User {
        active: true,
        email: String::from("hagepon@gmail.com"),
        username: String::from("Hagepon"),
        sign_in_count: 3,
    };
    println!("user1={}", user1.username);

    let mut user2 = User {
        active: true,
        email: String::from("hagepon@gmail.com"),
        username: String::from("Hagepon"),
        sign_in_count: 3,
    };
    user2.username = String::from("Mr.Hagepon");
    println!("user2={}", user2.username);

    let user3 = User {
        email: String::from("hagepon3@gmail.com"),
        ..user1
    };
    println!("user3={}", user3.email);
    // this is invalid the value has been moved.
    // println!("user1={}", user1.username);

    {
        let width1 = 30;
        let height1 = 50;
        println!("area={}", area(width1, height1));
    }
    {
        let rect1 = (30, 50);
        println!("area={}", area2(rect1));
    }
    {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        println!("rect={:?}", rect1);
        dbg!(&rect1);
        println!("area={}", area3(rect1));
    }
}
