
pub fn part1(input: &str) -> String {
    let mut sum: i64  = 0;
    let ranges: std::str::Split<'_, char> = input.trim().split(',');
    for range in ranges {
        let range = range.trim();
        if range.is_empty() {
            continue;
        }
        let range_comps: Vec<&str> = range.split('-').collect();
        if range_comps.len() != 2 {
            continue;
        }
        let start: i64 = match range_comps[0].trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };
        let end: i64 = match range_comps[1].trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        for i in start..=end {
            let split = i.to_string();
            let length = split.len();
            if length % 2 == 0 && split.as_bytes().get(0).unwrap().to_string()!="0"{
                let (p1,p2)= split.split_at(split.len()/2);
                if p1 == p2{
                    sum+=i;
                }
            }
        }

    }
    sum.to_string()
}

pub fn part2(input: &str) -> String {
    let mut sum: i64 = 0;
    let ranges: std::str::Split<'_, char> = input.trim().split(',');
    
    for range in ranges {
        let range = range.trim();
        if range.is_empty() {
            continue;
        }
        let range_comps: Vec<&str> = range.split('-').collect();
        if range_comps.len() != 2 {
            continue;
        }
        let start: i64 = match range_comps[0].trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };
        let end: i64 = match range_comps[1].trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        for num in start..=end {
            if is_invalid_part2(num) {
                sum += num;
            }
        }
    }
    
    sum.to_string()
}

fn is_invalid_part2(n: i64) -> bool {
    let s = n.to_string();
    
    // No leading zeros
    if s.starts_with('0') {
        return false;
    }
    
    let len = s.len();
    
    // Try all possible pattern lengths from 1 to len/2
    for pattern_len in 1..=len/2 {
        // Check if the total length is divisible by pattern length
        if len % pattern_len == 0 {
            let repetitions = len / pattern_len;
            // Must be repeated at least twice
            if repetitions >= 2 {
                let pattern = &s[0..pattern_len];
                let mut is_repeat = true;
                
                // Check if all repetitions match the pattern
                for i in 1..repetitions {
                    let start = i * pattern_len;
                    let end = start + pattern_len;
                    if &s[start..end] != pattern {
                        is_repeat = false;
                        break;
                    }
                }
                
                if is_repeat {
                    return true;
                }
            }
        }
    }
    
    false
}


