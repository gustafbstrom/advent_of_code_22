#[allow(dead_code)]
const NIY : &str = "Not implemented yet";

pub fn solution1(filename: &str) -> Result<i32, &str> {
    let s = std::fs::read_to_string(filename).unwrap();
    let chunks = s.split("\r\n\r\n");
    let mut v : Vec<Vec<i32>> = Vec::new();

    for chunk in chunks {
        let chunk = chunk.trim();
        
        let mut tmp : Vec<i32> = Vec::new();
        for number in chunk.split("\r\n") {
            let a : i32 = number.parse().unwrap();
            tmp.push(a);
        }
        v.push(tmp);
    }

    let maximum = v.iter().map(|v| v.iter().sum()).max().unwrap();

    Ok(maximum)
}

pub fn solution2(filename: &str) -> Result<i32, &str> {
    let s = std::fs::read_to_string(filename).unwrap();
    let chunks = s.split("\r\n\r\n");
    let mut v : Vec<Vec<i32>> = Vec::new();

    for chunk in chunks {
        let chunk = chunk.trim();
        
        let mut tmp : Vec<i32> = Vec::new();
        for number in chunk.split("\r\n") {
            let a : i32 = number.parse().unwrap();
            tmp.push(a);
        }
        v.push(tmp);
    }

    let mut v = v.iter().map(|v| v.iter().sum()).collect::<Vec<i32>>();
	v.sort_by(|a, b| b.cmp(a));
	let top_three = v.iter().take(3).sum();

    Ok(top_three)
}
