fn main() {
    proconio::input! {
        m: u32,
    }
    let result = solve(m);
    println!("{}", result);
}

fn solve(m: u32) -> String {
    let result: u32;
    if m < 100 {
        result = 0
    } else if m <= 5_000 {
        result = m / 100
    } else if  m <= 30_000 {
        result = ( m / 1000) + 50
    } else if m <= 70_000 {
        result = ( m / 1000 - 30) / 5 + 80
    } else {
        result = 89
    };
    
    format!("{:02}", result)
}
