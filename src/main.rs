use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let number: i32 = args[1].parse().unwrap();
    part1(number);
    part2(number);
}

fn part1(number: i32){
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut steps_taken: i32 = 0;
    let mut dir: i32 = 0;

    let mut max_steps: i32 = 1;
    let mut max_steps_taken: i32 = 0;

    for _i in 1..number{
        //right
        if dir == 0 {
            x = x + 1;
        }
        //up
        if dir == 1 {
            y = y + 1;
        }
        //left 
        if dir == 2 {
            x = x - 1;
        }
        //down
        if dir == 3 {
            y = y - 1;
        }

        steps_taken += 1;

        if steps_taken == max_steps{
            steps_taken = 0;
            max_steps_taken += 1;
            
            if max_steps_taken > 1 {
                max_steps += 1;
                max_steps_taken = 0;
            }
            
            dir += 1;

            if dir > 3{
                dir = 0
            }
        }
    }

    println!("Part 1 distance: {}", i32::abs(x - 0) + i32::abs(y - 0));
}

fn part2(number: i32){
    
    
    println!("Part 2: {}", number);
}