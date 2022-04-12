mod encode;
mod decode;

fn main() {
    /* to encode programs:

        convert instruction into it's Godel number using add/subtract
        add this number to a vector
        convert entire vector when finished using list_encoding()

    */

    let res = encode::reg_subtract(2, 3, 4);
    println!("{}", res);


    /* to decode programs:
    
        decode number into a list using decodeList
        decode each list element (each instr) into a double owl, and single owl if necessary
        (todo!() -> write a function to make this easier)
    
    */
}
