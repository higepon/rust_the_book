use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s);
}

fn main() {
    // panic!("oh no!");

    let f = File::open("hello.txt");
    /*
        let f = match f {
            Ok(file) => file,
            Err(error) => panic!("can't open"),
        };

    */
    // let f = File::open("hige.txt").expect("failed to open");
}
