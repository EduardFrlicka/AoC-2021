use std::fs;

pub fn _run() {
    let task_num = 3;

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
    let mut line_len = 0;
    let mut zeroes = Vec::new();
    let mut ones = Vec::new();
    let mut mask: usize = 0;
    for line in _content.lines().take(1) {
        line_len = line.as_bytes().len();
        for _ in 0..line_len {
            mask <<= 1;
            mask += 1;
            ones.push(0);
            zeroes.push(0);
        }
    }

    for line in _content.lines() {
        for (i, c) in line.as_bytes().iter().enumerate() {
            if *c as char == '1' {
                ones[i] += 1;
            } else {
                zeroes[i] += 1
            }
        }
    }

    let mut gamma: usize = 0;

    for i in 0..line_len {
        gamma *= 2;
        gamma += if ones[i] > zeroes[i] { 1 } else { 0 };
    }

    let epsilon = !gamma & mask;

    return (gamma * epsilon) as i32;
}

fn _task2(_content: &String) -> i32 {
    let mut idx: usize = 0;
    let mut lines = Vec::new();

    for line in _content.lines() {
        lines.push(line);
    }

    let line_len: usize = lines.last().unwrap().len().to_owned();

    while idx < line_len && lines.len() > 1 {
        let mut most_common = 0;
        for line in &lines {
            let c = line.as_bytes()[idx];
            if c as char == '1' {
                most_common += 1;
            } else {
                {
                    most_common -= 1;
                }
            }
        }
        let decide_char = if most_common >= 0 { '1' } else { '0' };

        lines = lines
            .iter()
            .filter(|&&x| x.as_bytes()[idx] as char == decide_char)
            .cloned()
            .collect();

        idx += 1;
    }

    let ox_word = lines.last().unwrap();

    let mut lines = Vec::new();

    for line in _content.lines() {
        lines.push(line);
    }

    idx = 0;

    while idx < line_len && lines.len() > 1 {
        let mut most_common = 0;
        for line in &lines {
            let c = line.as_bytes()[idx];
            if c as char == '1' {
                most_common += 1;
            } else {
                {
                    most_common -= 1;
                }
            }
        }
        let decide_char = if most_common < 0 { '1' } else { '0' };

        lines = lines
            .iter()
            .filter(|&&x| x.as_bytes()[idx] as char == decide_char)
            .cloned()
            .collect();

        idx += 1;
    }

    let co2_word = lines.last().unwrap();

    let mut ox = 0;
    let mut co2 = 0;

    for c in ox_word.as_bytes() {
        ox *= 2;
        ox += if *c as char == '1' { 1 } else { 0 };
    }

    for c in co2_word.as_bytes() {
        co2 *= 2;
        co2 += if *c as char == '1' { 1 } else { 0 };
    }

    return ox * co2;
}
