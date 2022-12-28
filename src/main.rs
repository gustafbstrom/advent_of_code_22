mod dec1;
mod dec2;
mod dec3;
mod dec4;
mod dec5;
mod dec6;

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
    
    match dec4::solution1("dec3_input.txt") {
        Ok(val) => println!("{}", val),
        Err(msg) => println!("{}", msg),
    }
    match dec4::solution2("dec3_input.txt") {
        Ok(val) => println!("{}", val),
        Err(msg) => println!("{}", msg),
    }

    match dec5::solution1_2("dec5_input.txt", true) {
        Ok(val) => println!("{}", val),
        Err(msg) => println!("{}", msg),
    }
    match dec5::solution1_2("dec5_input.txt", false) {
        Ok(val) => println!("{}", val),
        Err(msg) => println!("{}", msg),
    }

    match dec6::solution1("dec6_input.txt") {
        Ok(val) => println!("{}", val),
        Err(msg) => println!("{}", msg),
    }
    match dec6::solution2("dec6_input.txt", 14) {
        Ok(val) => println!("{}", val),
        Err(msg) => println!("{}", msg),
    }

}
