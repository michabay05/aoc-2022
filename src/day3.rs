use crate::read_file;

pub fn solution() {
    solution_1();
    solution_2();
}

const SAMPLE_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";


fn solution_1() {
    // Store the list of repeated elements
    let mut repeated: Vec<char> = vec![];

    // Read list from file
    let contents = read_file("inputs/day3.txt");
    let contents = contents.trim();
    // For every line of the input
    for el in contents.split('\n') {
        // Find the midpoint of the current line
        let n = el.len() / 2;
        // Split the current line using the midpoint
        let first_part = &el[0..n];
        let second_part = &el[n..];

        assert!(first_part.len() == second_part.len());

        let mut local_repeated: Vec<char> = vec![];
        // Loop over both the first and second part to find a repetition
        for fc in first_part.chars() {
            for sc in second_part.chars() {
                if fc == sc {
                    if !local_repeated.iter().any(|x| *x == fc) {
                        repeated.push(fc);
                        local_repeated.push(fc);
                    }
                }
            }
        }
    }
    
    // Get sum of priorities
    let mut sum = 0;
    for el in repeated {
        if el.is_uppercase() {
            sum += (el as u8 - ('A' as u8 - 1)) as u32 + 26;
        } else {
            sum += (el as u8 - ('a' as u8 - 1)) as u32;
        }
    }
    println!("Sum of repeated: {}", sum);
}

fn solution_2() {
    // Store the list of repeated elements
    let mut priorities: Vec<char> = vec![];

    // Read list from file
    let contents = read_file("inputs/day3.txt");
    let contents = contents.trim();

    let mut lines: [&str; 3] = [""; 3];
    for (ind, line) in contents.split("\n").enumerate() {
        // If not the end of a group, write current line into array
        if (ind + 1) % 3 != 0 {
            lines[(ind + 1) % 3] = line;
            continue;
        }
        lines[(ind + 1) % 3] = line;
        
        let mut local_priorities: Vec<char> = vec![];
        for c in lines[0].chars() {
            if lines[1].contains(c) && lines[2].contains(c) {
                // Found common character: common badge
                if !local_priorities.iter().any(|&x| x == c) {
                    local_priorities.push(c);
                    priorities.push(c);
                }
            }
        }
    }
    
    // Get sum of priorities
    let mut sum = 0;
    for el in priorities {
        if el.is_uppercase() {
            sum += (el as u8 - ('A' as u8 - 1)) as u32 + 26;
        } else {
            sum += (el as u8 - ('a' as u8 - 1)) as u32;
        }
    }
    println!("Sum of priorities: {}", sum);
}
