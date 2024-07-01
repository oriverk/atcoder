fn main() {
    proconio::input! {
        s: String,
    }

    let s: Vec<char> = s.chars().collect();
    if s[0] == s[1] && s[1] == s[2] {
        println!("Won")
    } else {
        println!("Lost")
    }
}
