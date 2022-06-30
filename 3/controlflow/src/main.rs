fn main() {
    let mut x: u128 = if true { 5 } else { 6 };
    let mut _y = 'label: loop {
        x += x;
        if x > 500 {
            println!("X is bigger than 5, {}", x);
            break 'label x * x;
        } else if x == 5 {
            println!("X is equal to 5, {}", x);
            break 1;
        } else {
            println!("X is not bigger than 5, {}", x);
            break 1;
        }
    };
    while _y > 0 {
        _y -= 1;
        println!("Y is {}", _y);
    }
    let a = [1, 2, 3, 4];
    let mut idx = 0;
    while idx < 4 {
        println!("{}", a[idx]);
        idx += 1;
    }
    for element in a {
        println!("-{}", element);
    }
    for i in (1..5).rev() {
        println!("--{}", i);
    }
}
