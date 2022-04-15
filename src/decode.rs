

pub fn decode_double_owl(num: u32) -> (u32, u32) {

    println!("DECODING DOUBLE OWL OF: {}", num);

    /* 0 not defined with double owl */
    if num == 0 {
        panic!("Zero not defined - something's gone wrong")
    }

    /* base case: 1 => <<0, 0>> */
    if num == 1 {
        return (0, 0)
    }

    //convert number to binary string
    let bin = format!("{num:b}");
    println!("Binary of encoded number: {}", bin);


    let mut rev = bin.chars().rev();

    let mut x: u32 = 0;
    let mut y: u32 = 0;

    loop {
        let next_num = rev.next();

        match next_num {
            Some('1') => {
                let substring = &rev.rev().collect::<String>();
                //convert the rest of the binary to y - if empty set to zero
                y = u32::from_str_radix(substring, 2).unwrap_or(0);
                break;
            },
            Some('0') => {
                x += 1;
                continue;
            },
            _ => panic!("Something broke!"),
        }

    }

    println!("Number of trailing zeros: {}", x);
    let y_bin = format!("{y:b}");
    println!("Rest of binary {} into decimal: {}", y_bin, y);

    (x, y)
}

pub fn decode_single_owl(num: u32) -> (u32, u32) {

    println!("DECODING SINGLE OWL!");

    //convert number to binary string
    let bin = format!("{num:b}");
    println!("Binary of {}: {}", num, bin);


    let mut rev = bin.chars().rev();

    //todo!() refactor out the while true - it's pretty ugly

    let mut x: u32 = 0;
    let mut y: u32 = 0;

    loop {
        let next_num = rev.next();

        match next_num {
            Some('0') => {
                let substring = &rev.rev().collect::<String>();
                //convert the rest of the binary to y - if empty set to zero
                y = u32::from_str_radix(substring, 2).unwrap_or(0);
                break;
            },
            Some('1') => {
                x += 1;
                continue;
            },
            _ => panic!("Something broke!"),
        }

    }

    println!("Number of trailing ones: {}", x);
    let y_bin = format!("{y:b}");
    println!("Rest of binary {} into decimal: {}", y_bin, y);

    (x, y)

}

pub fn decode_list(num: u32) -> Vec<u32> {

    let mut res: Vec<u32> = vec!();

    let bin = format!("{num:b}");

    let mut rev = bin.chars().rev();

    let mut val = 0;

    loop {
        let next_num = rev.next();

        match next_num {
            Some('0') => val += 1,
            Some('1') => {
                res.push(val);
                val = 0;
            },
            Some(_) => panic!("Something broke!"),
            None => break,
        }
    }

    if val > 0 {
        res.push(val);
    }

    res

}

//todo!() - write decodeProgram, making use of decode_list, decode_double_owl

use crate::instr::RegInstr;

pub fn decodeInstr(num: u32) -> RegInstr {

    //if 0, it's a Halt instruction
    if num == 0 {
        return RegInstr::Halt
    }

    //convert number into double owl
    let (a, b) = decode_double_owl(num);


    //if a is even, it's an add instruction
    if a % 2 == 0 {
        let i = a / 2;
        RegInstr::Add(i, b)
    } else {
        let i = (a - 1) / 2;
        let (j, k) = decode_single_owl(b);
        RegInstr::Sub(i, j, k)
    }   

}

mod tests {
    use crate::decode::{decode_double_owl, decode_single_owl, decode_list};

    #[test]
    fn decode_double_owl_test() {
        assert_eq!(decode_double_owl(34), (1, 8));
        assert_eq!(decode_double_owl(8), (3, 0));
        assert_eq!(decode_double_owl(276), (2, 34));
    }

    #[test]
    fn decode_single_owl_test() {
        assert_eq!(decode_single_owl(9), (1, 2));
        assert_eq!(decode_single_owl(71), (3, 4));
    }

    #[test]
    fn decode_list_test() {
        assert_eq!(decode_list(8), vec![3]);
        assert_eq!(decode_list(34), vec![1, 3]);
        assert_eq!(decode_list(276), vec![2, 1, 3]);
    }
}