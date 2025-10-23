use std::{fs, io};

fn do_stuff(fi: &str) -> Result<String, io::Error> {
    let s = fs::read_to_string(fi)?;
    Ok(s)
}

fn do_stuff_write(sit: &str) -> Result<(), io::Error> {
    fs::write("inputp2.txt", sit)?;
    Ok(())
}

fn problem1() {
    let mut s: &str;
    let cont: String;
    let mut longch = String::from("");
    let mut longby = String::from("");

    if let Ok(c) = do_stuff("p1.txt") {
        cont = c;
        s = &cont;
    } else {
        println!("Error reading file");
        return;
    }

    while let Some(i) = s.find('\n') {
        if s[..i].len() > longby.len() {
            longby.clear();
            longby = s[..i].to_string();
        }
        if s[..i].chars().count() > longch.chars().count() {
            longch.clear();
            longch = s[..i].to_string();
        }
        s = &s[i + 1..];
    }

    if s.len() > longby.len() {
        longby.clear();
        longby = s.to_string();
    }
    if s.chars().count() > longch.chars().count() {
        longch.clear();
        longch = s.to_string();
    }

    println!("Longest line by bytes is: '{longby}'");
    println!("Longest line by characthers is: '{longch}'");
}

fn rot13(sit: &str) -> Result<String, &str> {
    let mut transformers = String::with_capacity(sit.len());
    for c in sit.chars() {
        if !c.is_ascii() {
            return Err("Non-ASCII detected, conversion stopped.");
        }

        let t = match c {
            'A'..='Z' => (((c as u8 - b'A' + 13) % 26) + b'A') as char,
            'a'..='z' => (((c as u8 - b'a' + 13) % 26) + b'a') as char,
            _ => c,
        };
        transformers.push(t);
    }

    Ok(transformers)
}

fn problem2() {
    let cont: String;
    if let Ok(c) = do_stuff("p2.txt") {
        cont = c
    } else {
        println!("Error at read");
        return;
    }

    println!("The normal text is:");
    println!("{cont}\n");

    match rot13(&cont) {
        Ok(a) => {
            println! {"The encoded text is:\n{a}"};
            let _ = do_stuff_write(&a);
        }
        Err(b) => {
            println!("{b:?}");
        }
    }
}

fn abrev_corrector(sit: &str) -> String {
    let mut transformers = String::with_capacity(sit.len());
    for v in sit.split(" ") {
        let t = match v {
            "pt" | "ptr" => "pentru",
            "dl" => "domnul",
            "dna" => "doamna",
            _ => v,
        };
        transformers.push_str(t);
        transformers.push(' ');
    }
    transformers
}

pub fn problem3() {
    let cont: String;
    if let Ok(c) = do_stuff("p3.txt") {
        cont = c
    } else {
        println!("Error at read");
        return;
    }
    println!("The normal phrase is:");
    println!("{cont}\n");

    let s = abrev_corrector(&cont);
    println!("The corrected phrase is:\n'{s}'");
}

fn host_prework(sit: &str) {
    for line in sit.lines() {
        let line = line.trim();
        if line.starts_with("#") {
            continue;
        }

        let mut it = line.split_whitespace();

        let h1 = it.next();
        let h2 = it.next();

        if let (Some(h1), Some(h2)) = (h1, h2) {
            println!("{h2} => {h1}");
        }
    }
}

fn problem4() {
    let cont: String;
    if let Ok(c) = do_stuff("/etc/hosts") {
        cont = c
    } else {
        println!("Error at read");
        return;
    }

    host_prework(&cont);
}

fn main() {
    println!("Problem 1 output");
    problem1();

    println!("Problem 2 output");
    problem2();

    println!("Problem 3 output");
    problem3();

    println!("Problem 4 output");
    problem4();
}
