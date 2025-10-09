fn add_chars_n(mut s: String, c: char, x: i32) -> String {
    for _ in 0..x {
        s.push(c);
    }

    s
}

fn add_chars_n_p2(s: &mut String, c: char, x: i32) {
    for _ in 0..x {
        s.push(c);
    }
}

fn add_space(s: &mut String, x: i32) {
    for _ in 0..x {
        s.push(' ');
    }
}

fn add_str(mut s: String, c: &str) -> String {
    s += c;
    s
}

fn int_to_str(mut x: i32) -> String {
    let mut temp = String::from("");
    let neg = x < 0;

    if neg {
        x = -x;
    }

    while x > 0 {
        let d = (((x % 10) as u8) + b'0') as char;
        temp += &d.to_string();
        x /= 10;
    }

    if neg {
        temp.push('-');
    }

    let fin: String = temp.chars().rev().collect();
    temp.clear();

    if fin.len() < 3 {
        return fin;
    } else {
        if neg {
            temp.push('-');
        }
        let mut i: usize = if neg { 1 } else { 0 };
        let mut count = 0usize;
        while i < fin.len() {
            if count > 0 && count % 3 == 0 {
                temp.push('_');
            }
            temp.push_str(&fin[i..i + 1]);
            count += 1;
            i += 1;
        }
    }

    temp
}

fn float_to_str(x: f32) -> String {
    let int_p = x.trunc();
    let mut fra_p = x.fract();

    if fra_p < 0.0 {
        fra_p = -fra_p;
    }

    let mut num = int_to_str(int_p as i32);

    num.push('.');

    let mut fra_d = String::new();
    let mut i = 0;

    while i < 3 {
        fra_p *= 10.0;
        let d = fra_p.trunc() as u8;
        fra_d.push((b'0' + d) as char);
        fra_p -= d as f32;
        i += 1;
    }

    num.push_str(&fra_d);
    num
}

fn main() {
    let mut s = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + b'a') as char;
        s = add_chars_n(s, c, 26 - i);

        i += 1;
    }

    print!("P1 output: {s}\n\n");
    s.clear();

    let mut s = String::from("");
    i = 0;
    while i < 26 {
        let c = (i as u8 + b'a') as char;
        add_chars_n_p2(&mut s, c, 26 - i);

        i += 1;
    }

    println!("P2 output: {s}\n");

    println!("P3 output:\n");
    let mut p3 = String::from("");
    add_space(&mut p3, 49);
    p3 = add_str(p3, "I");
    add_space(&mut p3, 1);
    p3 = add_str(p3, "💚");
    println!("{p3}");

    p3.clear();
    add_space(&mut p3, 49);
    p3 = add_str(p3, "RUST.");
    println!("{p3}");

    p3.clear();
    add_space(&mut p3, 15);
    p3 = add_str(p3, "Most");
    add_space(&mut p3, 10);
    p3 = add_str(p3, "crate");
    add_space(&mut p3, 3);
    p3.push_str(&int_to_str(306437968));
    add_space(&mut p3, 9);
    p3 = add_str(p3, "and");
    add_space(&mut p3, 3);
    p3 = add_str(p3, "latest");
    add_space(&mut p3, 7);
    p3 = add_str(p3, "is");
    println!("{p3}");

    p3.clear();
    add_space(&mut p3, 15);
    add_space(&mut p3, 4);
    p3 = add_str(p3, "downloaded");
    add_space(&mut p3, 5);
    p3 = add_str(p3, "has");
    add_space(&mut p3, 11);
    p3 = add_str(p3, "downloads");
    add_space(&mut p3, 3);
    p3 = add_str(p3, "the");
    add_space(&mut p3, 6);
    p3 = add_str(p3, "version");
    add_space(&mut p3, 3);
    p3.push_str(&float_to_str(2.038));
    println!("{p3}");
}
