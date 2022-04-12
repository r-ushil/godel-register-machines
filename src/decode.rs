pub fn decodeDoubleOwl(num: u32) -> (u32, u32) {

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
    println!("{}", bin);


    let mut rev = bin.chars().rev();
    println!("{:#?}", rev);

    //todo!() refactor out the while true - it's pretty ugly

    let mut x: u32 = 0;
    let mut y: u32 = 0;

    while true {
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

    return (x, y)
}


mod tests {
    use super::decodeDoubleOwl;

    #[test]
    fn decodeDoubleOwlTest() {
        assert_eq!(decodeDoubleOwl(34), (1, 8));
        assert_eq!(decodeDoubleOwl(8), (3, 0));
        assert_eq!(decodeDoubleOwl(276), (2, 34));
    }
}