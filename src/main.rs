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

fn reg_subtract(i: u32, j: u32, k: u32) -> u32 {
    /* Ri- -> Lj, Lk converts to:
            << 2i + 1, <j, k> >>*/
    println!("CALCULATING SUBTRACT!");
    println!("Calculating single owl, for second arg in subtract's double owl");
    let snd = calcSingleOwl(j, k);
    let res = calcDoubleOwl(2*i + 1, snd);
    println!("R{}- -> L{}, L{} converts to {}", i, j, k, res);
    res
}

fn reg_add(i: u32, j: u32) -> u32 {
    /* Ri+ -> Lj converts to:
        <<2i, j>> */

    println!("CALCULATING ADD!");
    let res = calcDoubleOwl(2*i, j);
    println!("R{}+ -> L{} converts to {}", i, j, res);
    res
}

fn list_encoding(nums: Vec<u32>) -> u32 {

    /*  Empty list => 0
        x : l => <<x, `l`>>, where `l` is the rest of the encoded list */
    println!("ENCODING A LIST");

    if let Some(head) = nums.get(0) {
        //list is not empty
        if nums.len() > 1 {
            //list is not singleton
            let tail = list_encoding(nums[1..].to_vec());
            return calcDoubleOwl(*head, tail)
        } else {
            //list is singleton
            return calcDoubleOwl(*head, 0);
        }
    } else {
        //list is empty
        return 0
    }

}



fn main() {
    /* to encode programs:

        convert instruction into it's Godel number using add/subtract
        add this number to a vector
        convert entire vector when finished using list_encoding()

    */

    println!("Hello, world!");
}
