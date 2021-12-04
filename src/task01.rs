use std::fs;

pub fn _run() {
    let task_num = 1;

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
    let mut res = 0;
    let mut last = 0;
    for line in _content.lines() {
        let num = line.parse::<i32>().unwrap();
        if last == 0 {
            last = num;
            continue;
        }
        if last < num {
            res += 1;
        }
        last = num;
    }
    return res;
}

fn _task2(_content: &String) -> i32 {
    let mut res = 0;
    let mut last = 0;
    let mut sum;

    let mut mem = [0; 3];

    let mut state: usize = 0;

    for line in _content.lines() {
        let num = line.parse::<i32>().unwrap();
        mem[state % 3] = num;
        if state < 2 {
            state += 1;
            continue;
        }

        sum = mem.iter().sum();

        if last < sum {
            res += 1;
        }
        last = sum;
        state += 1;
    }
    return res - 1;
}
