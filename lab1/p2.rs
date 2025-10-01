//Make a function that calculates if two numbers are coprime, and call it with pairs of every number between 0 and 100.

fn are_coprime(mut nr: (i32, i32)) -> bool
{
    if nr.0 == 0 || nr.1 == 0 { return false; }
    while nr.0 != nr.1 {
        if nr.0 > nr.1 
        {
            nr.0 -= nr.1;
        }
        else {
            nr.1 -= nr.0;
        }
    }
    match nr.0 {
        1 => return true,
        _ => return false,
    }
}

fn main() {
    for i in 0..=100 {
        for j in 0..=100{
            match are_coprime((i, j)) {
                true => println!("The numbers {} and {} are coprime.", i, j),
                false => println!("The numbers {} and {} are not coprime.", i, j),
            };
        }
    }
}