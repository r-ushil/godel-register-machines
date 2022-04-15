use crate::instr::*;

pub fn calcDoubleOwl(x: u32, y: u32) -> u32 {
    /* 2 ^ x * ((2 * y) + 1) */

    println!("Calculating double owl for {}, {}", x, y);

    let base: u32 = 2;
    let res = base.pow(x) * ((2 * y) + 1);

    println!("<<{}, {}>> = 2^{} * (2 * {} + 1) = {}", x, y, x, y, res);
    res
}

pub fn calcSingleOwl(x: u32, y: u32) -> u32 {
    /* (2 ^ x * ((2 * y) + 1) - 1) */

    println!("Calculating single owl for {}, {}", x, y);
    let res = calcDoubleOwl(x, y) - 1;
    println!("Therefore, single owl is: <{}, {}> = {}", x, y, res);
    res
}

pub fn encode_instr(instr: RegInstr) -> u32 {
    match instr {
        RegInstr::Add(..) => reg_add(instr),
        RegInstr::Sub(..) => reg_subtract(instr),
        RegInstr::Halt => reg_halt(),
    }
}

pub fn reg_halt() -> u32{
    0
}

pub fn reg_subtract(sub_instr: RegInstr) -> u32 {

    if let RegInstr::Sub(i, j, k) = sub_instr {
        /* Ri- -> Lj, Lk converts to:
            << 2i + 1, <j, k> >>*/
        
        println!("CALCULATING SUBTRACT!");
        println!("Calculating single owl, for second arg in subtract's double owl");
        let snd = calcSingleOwl(j, k);
        let res = calcDoubleOwl(2*i + 1, snd);
        println!("R{}- -> L{}, L{} converts to {}", i, j, k, res);
        res
    } else {
        panic!("Wrong type of Register Instruction")
    }

    
}

pub fn reg_add(add_instr: RegInstr) -> u32 {

    if let RegInstr::Add(i, j) = add_instr {
        /* Ri+ -> Lj converts to:
        <<2i, j>> */

        println!("CALCULATING ADD!");
        let res = calcDoubleOwl(2*i, j);
        println!("R{}+ -> L{} converts to {}", i, j, res);
        res
    } else {
        panic!("Wrong type of Register Instruction")
    }
    
}

pub fn list_encoding(nums: Vec<u32>) -> u32 {
    /*  Empty list => 0
        x : l => <<x, `l`>>, where `l` is the rest of the encoded list */
    println!("ENCODING A LIST");

    if let Some(head) = nums.get(0) {
        //list is not empty
        if nums.len() > 1 {
            //list is not singleton
            let tail = list_encoding(nums[1..].to_vec());
            calcDoubleOwl(*head, tail)
        } else {
            //list is singleton
            calcDoubleOwl(*head, 0)
        }
    } else {
        //list is empty
        0
    }

}

#[cfg(test)]
mod tests {
    use crate::{encode::{encode_instr}, instr::RegInstr};

    use super::list_encoding;

    #[test]
    fn list_encoding_test() {
        assert_eq!(list_encoding(vec![3]), 8);
        assert_eq!(list_encoding(vec![1, 3]), 34);
        assert_eq!(list_encoding(vec![2, 1, 3]), 276);
    }

    #[test]
    fn reg_instr_test() {
        assert_eq!(encode_instr(RegInstr::Sub(1, 1, 2)), 152);
        assert_eq!(encode_instr(RegInstr::Add(0, 0)), 1);
        assert_eq!(encode_instr(RegInstr::Sub(2, 3, 4)), 4576);
        assert_eq!(encode_instr(RegInstr::Halt), 0);
        assert_eq!(encode_instr(RegInstr::Add(0, 2)), 5);
    }
}