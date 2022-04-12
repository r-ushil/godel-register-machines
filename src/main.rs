fn calcDoubleOwl(x: u32, y: u32) -> u32 {
    /* 2 ^ x * ((2 * y) + 1) */

    println!("Calculating double owl for {}, {}", x, y);

    let base: u32 = 2;
    let res = base.pow(x) * ((2 * y) + 1);

    println!("<<{}, {}>> = 2^{} * (2 * {} + 1) = {}", x, y, x, y, res);
    res
}

fn calcSingleOwl(x: u32, y: u32) -> u32 {
    /* (2 ^ x * ((2 * y) + 1) - 1) */

    println!("Calculating single owl for {}, {}", x, y);
    let res = calcDoubleOwl(x, y) - 1;
    println!("Therefore, single owl is: <{}, {}> = {}", x, y, res);
    res
}



fn main() {
    println!("Hello, world!");
}
