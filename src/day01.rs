fn car_cdr(s: &str) -> (&str, &str) {
    for i in 1..5 {
        let r = s.get(0..i);
        match r {
            Some(x) => return (x, &s[i..]),
            None => (),
        }
    }

    (&s[0..0], s)
}

pub fn part1(input: &str) -> String {
    let mut dial: i32 = 50;  // Dial starts at 50
    let mut zeros: i32 = 0;   // Count how many times dial points at 0
    
    for line in input.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        
        // Parse the rotation: L or R followed by a number
        let (side, n_str) = car_cdr(trimmed);
        let n = n_str.parse::<i32>().unwrap();
        
        // Apply rotation: L subtracts, R adds
        if side == "L" {
            dial -= n;
        } else {
            dial += n;
        }
        
        // Handle wrapping: dial is 0-99 (100 positions)
        // Use rem_euclid to handle negative numbers correctly
        dial = dial.rem_euclid(100);
        
        // Count if dial points at 0 after this rotation
        if dial == 0 {
            zeros += 1;
        }
    }
    
    zeros.to_string()
}


pub fn part2(input: &str) -> String {
    let mut dial: i32 = 50;  // Dial starts at 50
    let mut zeros: i32 = 0;   // Count how many times dial points at 0
    
    for line in input.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        
        // Parse the rotation: L or R followed by a number
        let (side, n_str) = car_cdr(trimmed);
        let n = n_str.parse::<i32>().unwrap();
        
        let start = dial;
        let final_pos = if side == "R" {
            start + n
        } else {
            start - n
        };
        
        // Count zeros during rotation (all steps except the final position)
        // For each step in the rotation, check if that position is 0 mod 100
        if side == "R" {
            // Rotating right: positions start+1, start+2, ..., start+n-1
            for step in 1..n {
                let pos = start + step;
                if pos % 100 == 0 {
                    zeros += 1;
                }
            }
        } else {
            // Rotating left: positions start-1, start-2, ..., start-n+1
            for step in 1..n {
                let pos = start - step;
                if pos % 100 == 0 {
                    zeros += 1;
                }
            }
        }
        
        // Check the final position (after the rotation, before wrapping)
        if final_pos % 100 == 0 {
            zeros += 1;
        }
        
        // Apply rotation
        if side == "L" {
            dial -= n;
        } else {
            dial += n;
        }
        
        // Handle wrapping: dial is 0-99 (100 positions)
        dial = dial.rem_euclid(100);
    }
    
    zeros.to_string()
}
