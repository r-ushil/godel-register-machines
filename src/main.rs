fn calcDoubleOwl(x: u32, y: u32) -> u32 {
    /* 2 ^ x * ((2 * y) + 1)) */

    let base: u32 = 2;
    base.pow(x) * ((2 * y) + 1)
}

fn calcSingleOwl(x: u32, y: u32) -> u32 {
    calcDoubleOwl(x, y) - 1
}

fn main() {
    println!("Hello, world!");
}
