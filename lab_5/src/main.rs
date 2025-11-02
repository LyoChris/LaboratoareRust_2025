use std::fs::{File};
use std::io::{self, BufReader, BufRead};
use serde_derive::Deserialize;

type PersonAge = (String, i32);
type OldestYoungest = (PersonAge, PersonAge);

fn to_i32(num: &str) -> i32 {
    let mut numb = 0i32;
    for c in num.chars() {
        if let '0'..='9' = c { numb = numb * 10 + ((c as u8 - b'0') as i32) }
    }

    numb
}

fn oldest_youngest(txt: &str) -> Result<OldestYoungest, io::Error> {
    let mut max: (String, i32) = (String::new(), -1);
    let mut min: (String, i32) = (String::new(), 9999);
    let file = File::open(txt)?;

    let read = BufReader::new(file);

    for line in read.lines() {
        let mut stud: (String, String, String) = (String::new(), String::new(), String::new());
        let line = line?;

        for (ind, word) in line.split(',').enumerate() {
            match ind {
                0 => stud.0 = word.to_string(),
                1 => stud.1 = word.to_string(),
                2 => stud.2 = word.to_string(),
                _ => (),
            }
        }
        let val = to_i32(&stud.2);
        if val > max.1 {
            max = (stud.0.clone(), val);
        }
        if val < min.1 {
            min = (stud.0.clone(), val);
        }
    }

    Ok((max, min))

}

fn problem_1() {
    let txt = String::from("students.txt");
    match oldest_youngest(&txt) {
        Ok((max, min)) => {
            println!("The oldest student is {} ({} years old).", max.0, max.1);
            println!("The youngest student is {} ({} years old).", min.0, min.1);
        },
        Err(e) => println!("Function failled. Cause: {e:?}"),
    }
}

fn set_pixels(c: &mut [[u8; 100]; 55], pixels: &[(u8, u8, u8)]) {
    for (x, y, val) in pixels {
        c[*x as usize][*y as usize] = *val;
    }
}

fn new_canvas() -> [[u8; 100]; 55] {
    [[b' '; 100]; 55]
}

fn print(canvas: [[u8; 100]; 55]) {
    for row in &canvas {
        for elem in row {
            print!("{0} ", *elem as char);
        }
        println!();
    }
}

fn problem_2() {
    let mut canvas = new_canvas();
    let c = &mut canvas;

    set_pixels(c, &[(4, 25, 124), (3, 33, 124), (2, 24, 95), (4, 3, 95)]);
    set_pixels(c, &[(7, 2, 95), (4, 21, 124), (5, 16, 95)]);
    set_pixels(c, &[(4, 41, 124), (7, 1, 124), (5, 8, 92)]);
    set_pixels(c, &[(1, 31, 40), (2, 3, 95), (2, 41, 124)]);
    set_pixels(c, &[(2, 16, 95), (5, 35, 92), (6, 3, 95), (2, 11, 95), (5, 3, 95)]);
    set_pixels(c, &[(2, 38, 95), (4, 9, 40), (3, 41, 124), (2, 37, 95), (2, 25, 124)]);
    set_pixels(c, &[(5, 27, 124), (2, 27, 124), (4, 0, 124), (3, 35, 47), (2, 18, 95)]);
    set_pixels(c, &[(4, 13, 124), (4, 37, 95), (4, 16, 40), (3, 6, 124)]);
    set_pixels(c, &[(7, 32, 47), (4, 20, 124), (5, 11, 95), (5, 42, 95)]);
    set_pixels(c, &[(5, 15, 92), (4, 34, 124), (4, 45, 41), (5, 24, 95)]);
    set_pixels(c, &[(4, 2, 40), (7, 3, 95), (2, 44, 95)]);
    set_pixels(c, &[(6, 30, 95), (5, 45, 95), (4, 31, 124), (4, 7, 124), (3, 43, 39)]);
    set_pixels(c, &[(5, 17, 95), (1, 27, 124), (2, 5, 95)]);
    set_pixels(c, &[(3, 44, 95), (3, 19, 92), (5, 23, 95), (3, 8, 47), (2, 10, 95)]);
    set_pixels(c, &[(6, 6, 124), (5, 19, 47), (3, 24, 95), (3, 27, 124)]);
    set_pixels(c, &[(3, 10, 95), (4, 44, 95), (2, 9, 95), (0, 32, 95), (5, 2, 95)]);
    set_pixels(c, &[(6, 2, 95), (7, 31, 95), (1, 25, 124), (2, 36, 95)]);
    set_pixels(c, &[(3, 46, 92), (5, 25, 44), (1, 43, 124), (5, 46, 47), (3, 15, 47)]);
    set_pixels(c, &[(4, 17, 95), (2, 23, 95), (3, 39, 92)]);
    set_pixels(c, &[(4, 47, 124), (2, 45, 95), (3, 37, 95)]);
    set_pixels(c, &[(5, 44, 95), (2, 2, 95), (5, 10, 95), (5, 9, 95), (4, 43, 124)]);
    set_pixels(c, &[(4, 38, 41), (2, 17, 95), (0, 26, 95)]);
    set_pixels(c, &[(4, 18, 41), (7, 5, 47), (5, 41, 124), (5, 33, 124)]);
    set_pixels(c, &[(5, 12, 47), (5, 22, 92), (6, 33, 124), (5, 31, 124)]);
    set_pixels(c, &[(4, 40, 124), (3, 3, 95), (4, 4, 124), (6, 31, 47), (3, 4, 96)]);
    set_pixels(c, &[(0, 42, 95), (5, 18, 95), (4, 27, 124)]);
    set_pixels(c, &[(3, 12, 92), (2, 32, 95), (5, 37, 95), (5, 26, 95), (5, 39, 47)]);
    set_pixels(c, &[(3, 25, 96), (4, 14, 124), (4, 33, 124), (3, 1, 47)]);
    set_pixels(c, &[(5, 36, 95), (7, 30, 95), (6, 4, 47), (4, 24, 95), (1, 32, 95)]);
    set_pixels(c, &[(3, 22, 47), (4, 23, 40), (5, 6, 124)]);
    set_pixels(c, &[(1, 33, 41), (1, 41, 124), (7, 29, 124)]);
    set_pixels(c, &[(4, 6, 124), (5, 38, 95), (3, 31, 124), (7, 4, 95)]);
    set_pixels(c, &[(4, 11, 41), (4, 10, 95), (5, 1, 92)]);
    set_pixels(c, &[(2, 43, 124), (3, 17, 95), (5, 4, 44), (4, 36, 40)]);
    set_pixels(c, &[(5, 43, 46)]);

    print(canvas);
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Student {
    name: String,
    phone: String,
    age: i32,
}

fn oldest_youngest_json(txt: &str) -> Result<OldestYoungest, io::Error> {
    let mut max: (String, i32) = (String::new(), -1);
    let mut min: (String, i32) = (String::new(), 9999);
    let file = File::open(txt)?;

    let read = BufReader::new(file);

    for line in read.lines() {
        let line = line?;
        let stud: Student = match serde_json::from_str(&line) {
            Ok(s) => s,
            Err(e) => {
                println!("Skipping invalid line: {e}");
                continue;
            }
        };

        if stud.age > max.1 {
            max = (stud.name.clone(), stud.age);
        }
        if stud.age < min.1 {
            min = (stud.name.clone(), stud.age);
        }
    }

    Ok((max, min))

}

fn problem_3() {
    let txt = String::from("p3.txt");
    match oldest_youngest_json(&txt) {
        Ok((max, min)) => {
            println!("The oldest student is {} ({} years old).", max.0, max.1);
            println!("The youngest student is {} ({} years old).", min.0, min.1);
        },
        Err(e) => println!("Function failled. Cause: {e:?}"),
    }
}


fn main() {
    println!("Problem 1 output:");
    problem_1();

    println!("Problem 2 output:");
    problem_2();

    println!("Problem 3 output:");
    problem_3();
}