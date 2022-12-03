mod dec1;
mod dec2;
mod dec3;

fn main() {
    match dec1::solution1("dec1_input.txt") {
        Ok(val) => println!("{}", val),
        Err(msg) => println!("{}", msg),
    }
    match dec1::solution2("dec1_input.txt") {
        Ok(val) => println!("{}", val),
        Err(msg) => println!("{}", msg),
    }

    match dec2::solution1("dec2_input.txt") {
        Ok(val) => println!("{}", val),
        Err(msg) => println!("{}", msg),
    }
    match dec2::solution2("dec2_input.txt") {
        Ok(val) => println!("{}", val),
        Err(msg) => println!("{}", msg),
    }

    match dec3::solution1("dec3_input.txt") {
        Ok(val) => println!("{}", val),
        Err(msg) => println!("{}", msg),
    }
    match dec3::solution2("dec3_input.txt") {
        Ok(val) => println!("{}", val),
        Err(msg) => println!("{}", msg),
    }

}
