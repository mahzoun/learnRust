enum Result<T, S> {
    Ok(T),
    Err(S),
}

fn largest<T: std::cmp::PartialOrd>(list: &mut [T]) -> &T {
    let mut biggest = &list[0];
    for number in list.iter() {
        if number > biggest {
            biggest = number;
        }
    }
    biggest
}

fn selection_sort<S: std::cmp::PartialOrd>(list: &mut [S]) -> &[S] {
    for i in 0..(list.len()) {
        let mut index_min = i;
        for j in (i)..(list.len()){
            if list[index_min] > list[j] {
                index_min = j;
            }
        }
        list.swap(i, index_min);
    }
    list
}
fn main() {
    let mut v = vec![4, 5, 3, 2, 10];
    let biggest = largest(&mut v);
    println!("{biggest}\n");
    selection_sort(&mut v);
    for i in v {
        println!("{i}");
    }
    let mut chars = vec!['d', 'z', 'a', 'c', 'b'];
    selection_sort(&mut chars);
    println!("Sorted chars: {:?}", chars);
}
