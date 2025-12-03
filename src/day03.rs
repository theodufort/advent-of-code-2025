pub fn part1(input: &str) -> String {
    let mut total_joltage = 0;
    
    for line in input.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        
        // Convert line to vector of digit values
        let digits: Vec<i32> = trimmed.chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect();
        
        // Find maximum joltage by checking all pairs of batteries
        let mut max_joltage = 0;
        for i in 0..digits.len() {
            for j in (i + 1)..digits.len() {
                // Form two-digit number: first digit * 10 + second digit
                let joltage = digits[i] * 10 + digits[j];
                max_joltage = max_joltage.max(joltage);
            }
        }
        
        println!("Line: {} -> Max joltage: {}", trimmed, max_joltage);
        total_joltage += max_joltage;
    }
    
    total_joltage.to_string()
}

pub fn part2(input: &str) -> String {
    let mut total_joltage = 0u64;
    
    for line in input.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        
        // Convert line to vector of digit values
        let digits: Vec<u32> = trimmed.chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        
        let n = digits.len();
        let k = 12; // need to pick 12 batteries
        
        // Greedy approach: for each position, pick the largest digit
        // that still allows us to pick k total digits
        let mut result = String::new();
        let mut current_idx = 0;
        
        for pos in 0..k {
            // We need to pick (k - pos) more digits including this one
            // So we can look ahead up to index: n - (k - pos)
            let max_look_ahead = n - (k - pos);
            
            // Find the largest digit in the valid range
            let mut max_digit = 0;
            let mut max_digit_idx = current_idx;
            
            for i in current_idx..=max_look_ahead {
                if digits[i] > max_digit {
                    max_digit = digits[i];
                    max_digit_idx = i;
                }
            }
            
            result.push_str(&max_digit.to_string());
            current_idx = max_digit_idx + 1;
        }
        
        let joltage: u64 = result.parse().unwrap();
        println!("Line: {} -> Max joltage: {}", trimmed, joltage);
        total_joltage += joltage;
    }
    
    total_joltage.to_string()
}


