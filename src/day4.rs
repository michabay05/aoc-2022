use crate::read_file;

pub fn solution() {
    solution1();
    solution2();
}

const SAMPLE_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

fn solution1() {
    let content = read_file("inputs/day4.txt");
    let content = content.trim();

    // Store the count of overlaps in assignment
    let mut count = 0;

    // Split content by lines then loop over every line
    for line in content.split('\n') {
        // Split line by comma to get assignment of each elf in a pair
        let mut pair = line.split(',');
        let first_elf = pair.next().unwrap();
        let second_elf = pair.next().unwrap();
        
        // Split the elf's assignment ranges
        let mut first_split = first_elf.split('-');
        let mut second_split = second_elf.split('-');

        // Get the max of the first and second elf
        let (fmin, fmax): (u32, u32) = (
            first_split.next().unwrap().parse().unwrap(),
            first_split.next().unwrap().parse().unwrap()
        );
        let (smin, smax): (u32, u32) = (
            second_split.next().unwrap().parse().unwrap(),
            second_split.next().unwrap().parse().unwrap()
        );

        if (smin <= fmin && fmax <= smax) || (fmin <= smin && smax <= fmax) {
            count += 1; 
        }
    }

    println!("Count of total overlaps: {count}");
}

fn solution2() {
    let content = read_file("inputs/day4.txt");
    let content = content.trim();

    // Store the count of overlaps in assignment
    let mut count = 0;

    // Split content by lines then loop over every line
    for line in content.split('\n') {
        // Split line by comma to get assignment of each elf in a pair
        let mut pair = line.split(',');
        let first_elf = pair.next().unwrap();
        let second_elf = pair.next().unwrap();
        
        // Split the elf's assignment ranges
        let mut first_split = first_elf.split('-');
        let mut second_split = second_elf.split('-');

        // Get the max of the first and second elf
        let (fmin, fmax): (u32, u32) = (
            first_split.next().unwrap().parse().unwrap(),
            first_split.next().unwrap().parse().unwrap()
        );
        let (smin, smax): (u32, u32) = (
            second_split.next().unwrap().parse().unwrap(),
            second_split.next().unwrap().parse().unwrap()
        );

        if (smin <= fmin && smax >= fmin) || (fmin <= smin && fmax >= smin){
            count += 1; 
        }
    }

    println!("  Count of any overlaps: {count}");
}
