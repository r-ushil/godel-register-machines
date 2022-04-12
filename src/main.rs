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

fn reg_subtract(i: u32, j: u32, k: u32) {
    /* Ri- -> Lj, Lk converts to:
            << 2i + 1, <j, k> >>*/
    println!("CALCULATING SUBTRACT!");
    println!("Calculating single owl, for second arg in subtract's double owl");
    let snd = calcSingleOwl(j, k);
    let res = calcDoubleOwl(2*i + 1, snd);
    println!("R{}- -> L{}, L{} converts to {}", i, j, k, res);
}

fn reg_add(i: u32, j: u32) {
    /* Ri+ -> Lj converts to:
        <<2i, j>> */

    println!("CALCULATING ADD!");
    let res = calcDoubleOwl(2*i, j);
    println!("R{}+ -> L{} converts to {}", i, j, res);
}



fn main() {
    println!("Hello, world!");
}
