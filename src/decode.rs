

pub fn decodeDoubleOwl(num: u32) -> (u32, u32) {

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
    let bin = format!("{:b}", num);
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
    let y_bin = format!("{:b}", y);
    println!("Rest of binary {} into decimal: {}", y_bin, y);

    return (x, y)
}

pub fn decodeSingleOwl(num: u32) -> (u32, u32) {

    println!("DECODING SINGLE OWL!");

    //convert number to binary string
    let bin = format!("{:b}", num);
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
    let y_bin = format!("{:b}", y);
    println!("Rest of binary {} into decimal: {}", y_bin, y);

    return (x, y)

}

pub fn decodeList(num: u32) -> Vec<u32> {

    let mut res: Vec<u32> = vec!();

    let bin = format!("{:b}", num);

    let mut rev = bin.chars().rev();

    let mut val = 0;

    loop {
        let next_num = rev.next();

        match next_num {
            Some('0') => val += 1,
            Some('1') => {
                res.push(val.clone());
                val = 0;
            },
            Some(_) => panic!("Something broke!"),
            None => break,
        }
    }

    if val > 0 {
        res.push(val.clone());
    }

    return res

}

//todo!() - write decodeProgram, making use of decodeList, decodeDoubleOwl


mod tests {
    use crate::decode::decodeList;

    use super::{decodeDoubleOwl, decodeSingleOwl};

    #[test]
    fn decodeDoubleOwlTest() {
        assert_eq!(decodeDoubleOwl(34), (1, 8));
        assert_eq!(decodeDoubleOwl(8), (3, 0));
        assert_eq!(decodeDoubleOwl(276), (2, 34));
    }

    #[test]
    fn decodeSingleOwlTest() {
        assert_eq!(decodeSingleOwl(9), (1, 2));
        assert_eq!(decodeSingleOwl(71), (3, 4));
    }

    #[test]
    fn decodeListTest() {
        assert_eq!(decodeList(8), vec![3]);
        assert_eq!(decodeList(34), vec![1, 3]);
        assert_eq!(decodeList(276), vec![2, 1, 3]);
    }
}