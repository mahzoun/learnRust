// TODO: remove this when you're done with your implementation.
// #![allow(unused_variables, dead_code)]

pub fn luhn(cc_number: &str) -> bool {
    let s:String = cc_number.chars().filter(|c| c.is_digit(10)).collect();
    let ss:String = cc_number.chars().filter(|c| !(c.is_whitespace() || c.is_digit(10))).collect();
    if s.len() < 2 || ss.len() > 0 {
        println!("fuck:{s}, {ss}");
        return false;
    }
    let digits: Vec<_> = s.chars().collect();
    let idx:usize = digits.len();
    let mut sum_of_digits = 0;
    for i in 0..(idx) {
        if (idx - i) % 2 == 0 {
            let double_digit = digits[i].to_digit(10).unwrap() * 2;
            let temp = (double_digit % 10) + (double_digit / 10);
            sum_of_digits += temp;
        } else {
            sum_of_digits+= digits[i].to_digit(10).unwrap();
        }
    }
    if sum_of_digits % 10 == 0 {
        true 
    } else {
        false
    }
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("fo  o"));
    assert!(!luhn("foo 0 0"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("                  6"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

// #[allow(dead_code)]
fn main() {

}