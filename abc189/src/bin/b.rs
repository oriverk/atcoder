use proconio::input;

fn main() {
    input! {
        n: u32,
        x: u32,
        vp: [(u32, u32); n],
    }

    let mut alcoholic = 0;
    for(i, (vi, pi)) in vp.iter().enumerate() {
            alcoholic += vi * pi;
            if x * 100 < alcoholic {
                println!("{}", i + 1);
                return
            }
    }
    println!("{}", -1);
}
