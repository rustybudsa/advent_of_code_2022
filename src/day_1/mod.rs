use std::fs;

pub fn start() {
    let theday = "day_1";

    println!("{}", theday);
    // let file_name = format!("/workspaces/DEV/advent_of_code_2022/src/{}/example", theday);
    let file_name = format!("/workspaces/DEV/advent_of_code_2022/src/{}/puzzle_input", theday);

    let file_contents = fs::read_to_string(file_name).expect("CANNOT read file");

    let inventory = file_contents.split("\n\n");

    let mut vec = Vec::new(); //for Part 2

    let mut max = 0;
    for cals in inventory {
        let cal = cals.split("\n");
        let mut sum = 0;
        for c in cal {
            let i = c.parse::<i32>().unwrap();
            // or
            // let i: i32 = c.parse().unwrap();
            sum += i;
            vec.push(sum); // for part2
        }
        if sum > max {
            max = sum;
        }
        // println!("sum {}", sum);
    }
    println!("answer part 1 = {}", max);
    println!();
    
    for x in &vec {
        println!("{x}");
    }
    vec.sort();
    println!();
    for x in &vec {
        println!("{x}");
    }

    println!();
    println!("answer part 2 = {}", max);
}
