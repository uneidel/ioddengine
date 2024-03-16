

pub fn cast_string_to_i8(val: String) -> i8 {
    let i8val: i8 = match val.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: Unable to parse string to i8!");
            
            return -1;
        }
    };
    i8val
}
