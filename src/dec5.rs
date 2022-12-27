
#[allow(dead_code)]
const NIY : &str = "Not implemented yet";

#[allow(unused_variables)]
pub fn solution1_2(filename: &str, rev: bool) -> Result<String, &str> {
    let lines = std::fs::read_to_string(filename).unwrap();
    
    // Compute no of crates
    let first = lines.split_once("\n").unwrap().0;
    let n_stacks = (first.len()+1) / 4;
    assert_eq!((first.len()+1) % 4, 0);
    let mut stacks = vec![Vec::new(); n_stacks];
    
    // Parse the stacks
    let mut line_n = 0;
    for line in lines.lines() {
        line_n += 1;
        if line == "" {
            break;
        }
        
        for i in (0..line.len()).step_by(4) {
            let c = line.chars().nth(i+1).unwrap();
            match c {
                ' ' => continue,
                '1' => break,
                _ => stacks[i/4].push(c)
            }
        }
    }
    stacks.iter_mut().for_each(|v| v.reverse());

    // Parse, execute the commands
    for line in lines.lines().skip(line_n) {
        let mut words = line.split_ascii_whitespace();
        let mv_from_to = (0..3u32)
            .map(|i| words
                .nth(1)
                .unwrap()
                .parse::<usize>()
                .unwrap())
            .collect::<Vec<usize>>();

        let (from_stack, to_stack) = {
            if mv_from_to[1] < mv_from_to[2] {
                let split = stacks
                .split_at_mut(mv_from_to[1]);
                (&mut split.0[mv_from_to[1]-1], &mut split.1[mv_from_to[2] - mv_from_to[1] - 1])
            }
            else {
                let split = stacks
                .split_at_mut(mv_from_to[2]);
                (&mut split.1[mv_from_to[1] - mv_from_to[2] - 1], &mut split.0[mv_from_to[2]-1])
            }
        };
        let drain = from_stack.drain(from_stack.len()-mv_from_to[0]..);
        if rev {
            to_stack.extend(drain.rev());
        }
        else {
            to_stack.extend(drain);
        }
    }

    Ok(stacks.iter().map(|v| v[v.len()-1]).collect::<String>())
}
