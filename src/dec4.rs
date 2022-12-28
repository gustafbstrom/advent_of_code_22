use regex;

#[allow(dead_code)]
const NIY : &str = "Not implemented yet";

#[allow(unused_variables)]
pub fn solution1(filename: &str) -> Result<i32, &str> {
    let s = std::fs::read_to_string(filename).unwrap();
    let re = regex::Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();

    let mut count = 0;
    for line in s.lines() {
        for cap in re.captures_iter(line) {
            let area_ranges = 
                {1..=4}
                .map(|i| cap[i].parse::<u32>().unwrap()).collect::<Vec<u32>>();
            if (area_ranges[0] >= area_ranges[2] && area_ranges[1] <= area_ranges[3])
                || (area_ranges[2] >= area_ranges[0] && area_ranges[3] <= area_ranges[1]) {
                    count += 1;
                }
        }
    }
    
    Ok(count)
}

#[allow(unused_variables)]
pub fn solution2(filename: &str) -> Result<i32, &str> {
    let s = std::fs::read_to_string(filename).unwrap();
    let re = regex::Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();

    let mut count = 0;
    for line in s.lines() {
        for cap in re.captures_iter(line) {
            let area_ranges = 
                {1..=4}
                .map(|i| cap[i].parse::<u32>().unwrap()).collect::<Vec<u32>>();
            if (area_ranges[2]..=area_ranges[3]).contains(&area_ranges[0])
                || (area_ranges[2]..=area_ranges[3]).contains(&area_ranges[1])
                || (area_ranges[0]..=area_ranges[1]).contains(&area_ranges[2])
                || (area_ranges[0]..=area_ranges[1]).contains(&area_ranges[3]) {
                    count += 1;
                }
        }
    }

    Ok(count)
}
