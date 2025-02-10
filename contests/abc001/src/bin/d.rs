use std::iter::FromIterator;

use im_rc::HashSet;

fn main() {
    proconio::input! {
        n: u8,
        memo: [String; n]
    }

    let vec = solve(&memo);
    for item in &vec {
        println!("{}", item);
    }
}

fn solve(memo: &Vec<String>) -> Vec<String> {
    let unique: HashSet<String> = memo.into_iter().collect();
    let mut vec_unique = Vec::from_iter(unique);
    if vec_unique.len() == 1 {
        return vec_unique;
    }
    vec_unique.sort();

    for item in &vec_unique {
        let parts: Vec<&str> = item.split("-").collect();
        let from = parts[0];
        let to = parts[1];

    }
    vec_unique
}

fn round_time(time: String, isRoundup: bool) -> String {

    "hoge".to_string()
}
