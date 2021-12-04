use std::fs;

pub fn _run() {
    let task_num = 2;

    println!("Task {:02}:", task_num);

    let input_file = format!("inputs/input{:02}.txt", task_num);
    // let input_file = "inputs/test.txt";

    let content = fs::read_to_string(input_file).expect("Chyba");

    let res1 = _task1(&content);
    println!("  1: {}", res1);

    let res2 = _task2(&content);
    println!("  2: {}", res2);
}

fn _task1(_content: &String) -> i32 {
    let mut x = 0;
    let mut y = 0;

    for line in _content.lines() {
        let (dir, val) = line.split_once(' ').unwrap();
        let val = val.parse::<i32>().unwrap();

        if dir == "forward" {
            x += val;
        }
        if dir == "down" {
            y += val;
        }
        if dir == "up" {
            y -= val;
        }
    }

    return x * y;
}

fn _task2(_content: &String) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    for line in _content.lines() {
        let (dir, val) = line.split_once(' ').unwrap();
        let val = val.parse::<i32>().unwrap();

        if dir == "forward" {
            x += val;
            y += aim * val;
        }
        if dir == "down" {
            aim += val;
        }
        if dir == "up" {
            aim -= val;
        }
    }
    return x*y;
}
