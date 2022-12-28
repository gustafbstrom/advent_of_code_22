#[allow(dead_code)]
const NIY : &str = "Not implemented yet";

fn aux_naive(s: &str, win_size: usize) -> Option<i32> {
    let mut window = String::new();
    'outer: for i in 0..s.len()-win_size-1 {
        let ref ss = s[i..i+win_size];
        ss.clone_into(&mut window);
        let mut chars = ss.chars().collect::<Vec<char>>();
        chars.sort();
        for i in 1..chars.len() {
            if chars[i-1] == chars[i] {
                continue 'outer;
            }
        }
        return Some((i + win_size) as i32);
    }

    None // No eligible substring was found
}

pub fn solution1(filename: &str) -> Result<i32, &str> {    
    assert_eq!(aux_naive("aaaaaaaaaaaaaaaaaaa", 4), None);
    assert_eq!(aux_naive("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), Some(7));
    assert_eq!(aux_naive("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), Some(5));
    assert_eq!(aux_naive("nppdvjthqldpwncqszvftbrmjlhg", 4), Some(6));
    assert_eq!(aux_naive("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), Some(10));
    assert_eq!(aux_naive("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), Some(11));

    let lines = std::fs::read_to_string(filename).unwrap();
    
    if let Some(num) = aux_naive(&lines, 4) {
        return Ok(num);
    }
    Err("No substring found")
}

#[allow(unused_variables)]
pub fn solution2(filename: &str, win_size: usize) -> Result<i32, &str> {
    assert_eq!(aux_naive("mjqjpqmgbljsphdztnvjfqwrcgsmlb", win_size), Some(19));
    assert_eq!(aux_naive("bvwbjplbgvbhsrlpgdmjqwftvncz", win_size), Some(23));
    assert_eq!(aux_naive("nppdvjthqldpwncqszvftbrmjlhg", win_size), Some(23));
    assert_eq!(aux_naive("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", win_size), Some(29));
    assert_eq!(aux_naive("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", win_size), Some(26));

    let lines = std::fs::read_to_string(filename).unwrap();
    
    if let Some(num) = aux_naive(&lines, win_size) {
        return Ok(num);
    }
    Err("No substring found")
}
