fn main() {
    let s = String::from("an apple a day keep doctor away");
    let mut ss = String::from("");
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    for word in s.split_whitespace(){
        let first_character = word.chars().nth(0);
        let mut seen = false;
        for c in vowels {
            if first_character == Some(c) {
                seen = true;
            }
        }
        if !seen {
            ss = ss + &word[1..] + &word[..1] + "ay";
        } else {
            ss = ss + &word + "ay";
        }
        ss = ss + " ";
    }
    println!("{ss}");
}
