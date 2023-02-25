use crate::read_file;

pub fn solution() {
    let contents = read_file("input/day1.txt");

    let split_words = contents.trim().split("\n");

    let mut elf_calories: Vec<u32> = vec![];

    let mut curr_calorie_count: u32 = 0;
    for word in split_words {
        if word.is_empty() {
            elf_calories.push(curr_calorie_count);
            curr_calorie_count = 0;
            continue;
        }
        curr_calorie_count += word.parse::<u32>().unwrap();
    }

    println!("Calories\n{:?}", elf_calories);
    println!("\nCount = {}\n", elf_calories.len());
    elf_calories.sort();
    println!("Calories\n{:?}", elf_calories);
    elf_calories.reverse();

    let top_three_calories = elf_calories[0] + elf_calories[1] + elf_calories[2];
    println!("Top three calorie count: {}" , top_three_calories);
}
