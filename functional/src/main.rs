fn main() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure {:?}", list);

    let mut only_bollows = || list.push(7);
    //println!("before calls {:?}", list);
    only_bollows();
    println!("after call {:?}", list);

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    {
        let v1: Vec<i32> = vec![1, 2, 3];
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
        println!("v2={:?}", v2[0]);
    }
}
