fn main() {
    proconio::input! {
        s: String,
    }

    resolve2(s);
}

#[allow(dead_code)]
fn resolve(s: String) {
    let s: Vec<char> = s.chars().collect();
    let mut rice = 0;
    let mut miso = 0;
    for i in 0..=2 {
        if s[i] == 'R' {
            rice = i;
        } else if s[i] == 'M' {
            miso = i;
        }
    }
    if rice < miso {
        println!("Yes");
    } else {
        println!("No");
    }
}

#[allow(dead_code)]
fn resolve1(s: String) {
    let ri = s.find("R").unwrap();
    let mi = s.find("M").unwrap();
    println!("{}", if ri < mi { "Yes" } else { "No" });
}

#[allow(dead_code)]
fn resolve2(s: String) {
    if s.contains("RM") || s.contains("RSM") {
        println!("Yes");
    } else {
        println!("No");
    }
}
