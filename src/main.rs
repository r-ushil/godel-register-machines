mod encode;

fn main() {
    /* to encode programs:

        convert instruction into it's Godel number using add/subtract
        add this number to a vector
        convert entire vector when finished using list_encoding()

    */

    let res = encode::reg_subtract(2, 3, 4);
    println!("{}", res);

    println!("Hello, world!");
}
