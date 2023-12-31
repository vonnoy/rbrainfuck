use std::{
    collections::HashMap,
    io::{self, Write},
};

fn execute(
    code: &Vec<char>,
    pc: &mut usize,
    memory: &mut Vec<u8>,
    dc: &mut usize,
    brackets_cache: &mut HashMap<usize, usize>,
) {
    match code[*pc] {
        '>' => *dc += 1,
        '<' => *dc -= 1,
        '+' => memory[*dc] = memory[*dc].wrapping_add(1),
        '-' => memory[*dc] = memory[*dc].wrapping_sub(1),
        '.' => {
            print!("{}", memory[*dc] as char);
            io::stdout().flush().unwrap();
        }
        ',' => {
            let mut temp_str = String::new();
            io::stdin().read_line(&mut temp_str).unwrap();
            memory[*dc] = temp_str.chars().next().unwrap() as u8;
        }
        '[' => {
            if memory[*dc] == 0 {
                *pc = *brackets_cache.get(pc).unwrap();
            }
        }
        ']' => {
            if memory[*dc] != 0 {
                *pc = *brackets_cache.get(pc).unwrap();
            }
        }
        _ => (),
    }
    *pc += 1
}

fn fill_brackets_cache(code: &Vec<char>, brackets_cache: &mut HashMap<usize, usize>) {
    let mut stack = vec![];

    for index in 0..code.len() {
        match code[index] {
            '[' => stack.push(index),
            ']' => {
                let left = match stack.pop() {
                    Some(i) => i,
                    _ => panic!("Unmatched brackets as position: {}", index),
                };
                brackets_cache.insert(left, index);
                brackets_cache.insert(index, left);
            }
            _ => (),
        }
    }
}

pub fn interpret(code_buffer: &String) {
    let mut memory: Vec<u8> = vec![0; 10240];
    let mut memory_pointer: usize = 0;

    let code: Vec<char> = code_buffer.chars().collect();
    let mut code_pointer: usize = 0;

    let mut brackets_cache: HashMap<usize, usize> = HashMap::new();
    fill_brackets_cache(&code, &mut brackets_cache);

    loop {
        execute(
            &code,
            &mut code_pointer,
            &mut memory,
            &mut memory_pointer,
            &mut brackets_cache,
        );

        if code_pointer >= code.len() {
            break;
        }
    }
}
