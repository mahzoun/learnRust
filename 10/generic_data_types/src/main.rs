fn largest(list: &mut [i32]) -> &i32 {
    list[1] = 100;
    let mut biggest = &list[0];
    for number in list.iter() {
        if number > biggest {
            biggest = number;
        }
    }
    biggest
}

fn selection_sort(list: &mut [i32]) -> &[i32] {
    for i in 0..(list.len()) {
        let mut index_min = i;
        for j in (i)..(list.len()){
            if list[index_min] > list[j] {
                index_min = j;
            }
        }
        let temp = list[i];
        list[i] = list[index_min];
        list[index_min] = temp;
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
}
