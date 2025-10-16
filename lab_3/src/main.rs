fn is_prime(x: u32) -> bool {
    for d in 2..x / 2 {
        if x % d == 0 {
            return false;
        }
    }

    true
}

fn next_prime(x: u16) -> Option<u16> {
    let mut u: u32 = x as u32;
    while u < u16::MAX as u32 {
        if is_prime(u) {
            return Some(u as u16);
        }
        u += 1;
    }

    None
}

#[derive(Debug)]
enum MathErr {
    Overflow,
}

use std::io::{self, Write};
use std::panic;

fn add_u32(x: u32, y: u32) -> Result<u32, MathErr> {
    let res: u64 = x as u64 + y as u64;
    match u32::try_from(res) {
        Ok(res) => Ok(res),
        Err(_res) => Err(MathErr::Overflow),
    }
}

fn mul_u32(x: u32, y: u32) -> Result<u32, MathErr> {
    let res: u64 = x as u64 * y as u64;
    match u32::try_from(res) {
        Ok(res) => Ok(res),
        Err(_res) => Err(MathErr::Overflow),
    }
}

fn add_u32_panic(x: u32, y: u32) -> Result<u32, MathErr> {
    let res: u64 = x as u64 + y as u64;
    match u32::try_from(res) {
        Ok(res) => Ok(res),
        Err(_res) => panic!("Overflow"),
    }
}

fn mul_u32_panic(x: u32, y: u32) -> Result<u32, MathErr> {
    let res: u64 = x as u64 * y as u64;
    match u32::try_from(res) {
        Ok(res) => Ok(res),
        Err(_res) => panic!("Overflow"),
    }
}

fn mul_of_sum(x: u32, y: u32, z: u32) -> Result<u32, MathErr> {
    let sum = add_u32(x, y)?;
    let prod = mul_u32(sum, z)?;
    Ok(prod)
}

#[derive(Debug)]
enum FuncError {
    NotAscii,
    NotDigit,
    NotBase16Digit,
    NotLetter,
    NotPrintable,
}

fn to_uppercase(ch: char) -> Result<char, (FuncError, char)> {
    match ch {
        'a'..='z' => Ok(((ch as u8) - (b'a' - b'A')) as char),
        'A'..='Z' => Ok(ch),
        _ => Err((FuncError::NotLetter, ch)),
    }
}

fn to_lowercase(ch: char) -> Result<char, (FuncError, char)> {
    match ch {
        'a'..='z' => Ok(ch),
        'A'..='Z' => Ok(((ch as u8) + (b'a' - b'A')) as char),
        _ => Err((FuncError::NotLetter, ch)),
    }
}

fn print_char(ch: char) -> Result<char, (FuncError, char)> {
    match ch.is_ascii_graphic() {
        true => Ok(ch),
        false => Err((FuncError::NotPrintable, ch)),
    }
}

fn char_to_number(ch: char) -> Result<u8, (FuncError, char)> {
    if !ch.is_ascii() {
        return Err((FuncError::NotAscii, ch));
    }
    match ch {
        '0'..='9' => Ok(ch as u8 - b'0'),
        _ => Err((FuncError::NotDigit, ch)),
    }
}

fn char_to_number_hex(ch: char) -> Result<u8, (FuncError, char)> {
    if !ch.is_ascii() {
        return Err((FuncError::NotAscii, ch));
    }
    match ch {
        '0'..='9' => Ok(ch as u8 - b'0'),
        'A'..='F' => Ok((ch as u8 - b'A') + 10u8),
        _ => Err((FuncError::NotBase16Digit, ch)),
    }
}

fn print_error(er: (FuncError, char)) {
    match er.0 {
        FuncError::NotAscii => println!("The characther '{}' is not ASCII.", er.1),
        FuncError::NotDigit => println!("The characther '{}' is not a digit", er.1),
        FuncError::NotBase16Digit => println!("The characther '{}' is not a base 16 digit.", er.1),
        FuncError::NotLetter => println!("The characther '{}' is not a letter.", er.1),
        FuncError::NotPrintable => println!("The characther {:?} is not printable.", er.1),
    }
}

fn to_uppercase_n(ch: char) -> Result<char, FuncError> {
    match ch {
        'a'..='z' => Ok(((ch as u8) - (b'a' - b'A')) as char),
        'A'..='Z' => Ok(ch),
        _ => Err(FuncError::NotLetter),
    }
}

fn to_lowercase_n(ch: char) -> Result<char, FuncError> {
    match ch {
        'a'..='z' => Ok(ch),
        'A'..='Z' => Ok(((ch as u8) + (b'a' - b'A')) as char),
        _ => Err(FuncError::NotLetter),
    }
}

fn text_to_uppercase(text: &str) -> Result<String, FuncError> {
    let mut modi = String::new();
    for c in text.chars() {
        let s = to_uppercase_n(c)?;
        modi.push(s);
    }
    Ok(modi)
}

fn text_to_lowercase(text: &str) -> Result<String, FuncError> {
    let mut modi = String::new();
    for c in text.chars() {
        let s = to_lowercase_n(c)?;
        modi.push(s);
    }
    Ok(modi)
}

fn text_to_lowercase_first(text: &str) -> Result<String, FuncError> {
    let mut modi = String::new();
    let mut chars = text.chars();

    let first = match chars.next() {
        Some(c) => to_lowercase_n(c)?,
        None => return Ok(String::new()),
    };

    modi.push(first);
    modi.extend(chars);

    Ok(modi)
}

fn text_to_uppercase_first(text: &str) -> Result<String, FuncError> {
    let mut modi = String::new();
    let mut chars = text.chars();

    let first = match chars.next() {
        Some(c) => to_uppercase_n(c)?,
        None => return Ok(String::new()),
    };

    modi.push(first);
    modi.extend(chars);

    Ok(modi)
}

fn main() {
    let good = 25u16;
    let bad = u16::MAX;

    println!("P1 tests:");
    match next_prime(good) {
        Some(x) => println!("The next prime after {good} is {x}"),
        None => println!("There is not next prime after {good} that fits in u16."),
    };

    match next_prime(bad) {
        Some(x) => println!("The next prime after {bad} is {x}"),
        None => println!("There is not next prime after {bad} that fits in u16."),
    };

    println!("P2 tests:");
    let a = 35u32;
    let b = 25u32;
    let c = u32::MAX;

    let result = panic::catch_unwind(|| add_u32_panic(a, b));
    match result {
        Ok(res) => println!("The sum of {a} and {b} is {res:?}"),
        Err(_) => println!("Program panicked. The sum of {a} and {b} counldn't be computed."),
    }

    let result = panic::catch_unwind(|| add_u32_panic(b, c));
    match result {
        Ok(res) => println!("The sum of {b} and {c} is {res:?}"),
        Err(_) => println!("Program panicked. The sum of {b} and {c} counldn't be computed."),
    }

    let result = panic::catch_unwind(|| mul_u32_panic(a, b));
    match result {
        Ok(res) => println!("The multiplication of {a} and {b} is {res:?}."),
        Err(_) => {
            println!("Program panicked. The multiplication of {a} and {b} counldn't be computed")
        }
    }

    let result = panic::catch_unwind(|| add_u32_panic(b, c));
    match result {
        Ok(res) => println!("The multiplication of {b} and {c} is {res:?}."),
        Err(_) => {
            println!("Program panicked. The multiplication of {b} and {c} counldn't be computed")
        }
    }

    println!("P3 tests:");
    let x = 35u32;
    let y = 25u32;
    let z = u32::MAX;
    let t = 5u32;

    match add_u32(x, y) {
        Ok(a) => println!("The sum of {x} and {y} is {a}."),
        Err(a) => println!("The sum of {x} and {y} counldn't be computed. Reason: {a:?}"),
    };

    match add_u32(z, y) {
        Ok(a) => println!("The sum of {z} and {y} is {a}."),
        Err(a) => println!("The sum of {z} and {y} counldn't be computed. Reason: {a:?}"),
    };

    match mul_u32(x, y) {
        Ok(a) => println!("The multiplication of {x} and {y} is {a}."),
        Err(a) => {
            println!("The multiplication of {x} and {y} counldn't be computed. Reason: {a:?}")
        }
    };

    match mul_u32(z, y) {
        Ok(a) => println!("The multiplication of {z} and {y} is {a}."),
        Err(a) => {
            println!("The multiplication of {z} and {y} counldn't be computed. Reason: {a:?}")
        }
    };

    match mul_of_sum(x, y, t) {
        Ok(a) => println!("({x}+{y})*{t} is {a}."),
        Err(a) => println!("({x}+{y})*{t} counldn't be computed. Reason: {a:?}"),
    };

    match mul_of_sum(x, y, z) {
        Ok(a) => println!("({x}+{y})*{z} is {a}."),
        Err(a) => println!("({x}+{y})*{z} counldn't be computed. Reason: {a:?}"),
    };

    println!("P4 tests:");

    match to_uppercase('b') {
        Ok(c) => println!("Uppercase of 'b' is '{c}'."),
        Err(e) => print_error(e),
    }
    match to_uppercase('C') {
        Ok(c) => println!("Uppercase of 'C' is '{c}'."),
        Err(e) => print_error(e),
    }
    match to_uppercase('.') {
        Ok(c) => println!("Uppercase of '.' is '{c}'."),
        Err(e) => print_error(e),
    }

    match to_lowercase('G') {
        Ok(c) => println!("Lowercase of 'G' is '{c}'."),
        Err(e) => print_error(e),
    }
    match to_lowercase('b') {
        Ok(c) => println!("Lowercase of 'b' is '{c}'."),
        Err(e) => print_error(e),
    }
    match to_lowercase('/') {
        Ok(c) => println!("Lowercase of '/' is '{c}'."),
        Err(e) => print_error(e),
    }

    match print_char('~') {
        Ok(c) => println!("Printable char: '{c}'."),
        Err(e) => print_error(e),
    }
    match print_char('\n') {
        Ok(c) => println!("Printable char: '{c}'."),
        Err(e) => print_error(e),
    }

    match char_to_number('8') {
        Ok(c) => println!("The conversion of '8' to digit is {c}."),
        Err(e) => print_error(e),
    }
    match char_to_number('x') {
        Ok(c) => println!("The conversion of '8' to digit is {c}."),
        Err(e) => print_error(e),
    }

    match char_to_number_hex('F') {
        Ok(c) => println!("The conversion of 'F' to base 16 digit is {c}."),
        Err(e) => print_error(e),
    }
    match char_to_number_hex('z') {
        Ok(c) => println!("The conversion of 'z' to base 16 digit is {c}."),
        Err(e) => print_error(e),
    }
    match char_to_number_hex('8') {
        Ok(c) => println!("The conversion of '8' to base 16 digit is {c}."),
        Err(e) => print_error(e),
    }

    println!("P5 output:");
    let mut input = String::new();
    loop {
        input.clear();

        print!("Enter text:");
        let _ = io::stdout().flush();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let parts: Vec<&str> = input.split_whitespace().collect();

        if parts.len() < 2 {
            println!(
                "Incorrect usage: U/u/Uu/uU <text> \n U makes all text uppercase \n l makes all text lowercase \n F makes only the first letter Uppercase \n f makes only the first letter lowercase \n Q to quit"
            );
        }

        match parts[0] {
            "U" => {
                let txt = match text_to_uppercase(parts[1]) {
                    Ok(val) => val,
                    Err(e) => {
                        println!("Error: {e:?}");
                        return;
                    }
                };
                println!("Processed text is: {txt}");
            }
            "l" => {
                let txt = match text_to_lowercase(parts[1]) {
                    Ok(val) => val,
                    Err(e) => {
                        println!("Error: {e:?}");
                        return;
                    }
                };
                println!("Processed text is: {txt}");
            }
            "F" => {
                let txt = match text_to_uppercase_first(parts[1]) {
                    Ok(val) => val,
                    Err(e) => {
                        println!("Error: {e:?}");
                        return;
                    }
                };
                println!("Processed text is: {txt}");
            }
            "f" => {
                let txt = match text_to_lowercase_first(parts[1]) {
                    Ok(val) => val,
                    Err(e) => {
                        println!("Error: {e:?}");
                        return;
                    }
                };
                println!("Processed text is: {txt}");
            }
            "Q" => {
                println!("Quiting");
                return;
            }
            _ => println!("Unknown command"),
        }
    }
}
