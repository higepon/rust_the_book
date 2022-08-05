fn find_largest(number_list: &[i32]) -> i32 {
    let mut largest = number_list[0];

    for &number in number_list {
        if number > largest {
            largest = number;
        }
    }
    return largest;
}

fn find_largest2<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    return largest;
}
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("largest is {largest}");
    let number_list = vec![34, 50, 25, 100, 65];
    let largest = find_largest(&number_list);
    println!("largest is {largest}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = find_largest2(&char_list);
    println!("The largest char is {}", result);

    let number_list = vec![34, 50, 25, 100, 65];
    let result = find_largest2(&number_list);
    println!("The largest number is {}", result);
}
