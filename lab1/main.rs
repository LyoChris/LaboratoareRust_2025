//Make a function that calculates if a number is prime, and call it with every number from 0 to 100, printing the primes.
fn is_prime(x: i32) -> bool {
    if x == 0 || x == 1 {
        return false;
    }
    for i in 2..x / 2 {
        if x % i == 0 {
            return false;
        }
    }
    return true;
}

//Make a function that calculates if two numbers are coprime, and call it with pairs of every number between 0 and 100.
fn are_coprime(mut nr: (i32, i32)) -> bool {
    if nr.0 == 0 || nr.1 == 0 {
        return false;
    }
    while nr.0 != nr.1 {
        if nr.0 > nr.1 {
            nr.0 -= nr.1;
        } else {
            nr.1 -= nr.0;
        }
    }
    match nr.0 {
        1 => return true,
        _ => return false,
    }
}

//"Sing" the 99 bottles of beer problem.
fn bottles_of_beers() {
    let mut n = 99i32;
    while n > 0 {
        println!(
            "{0} bottles of beer on the wall,\n{0} bottles of beer.\nTake one down, pass it around,",
            n
        );
        n -= 1;
        if n > 0 {
            println!("{} bottles of beer on the wall.\n", n);
        } else {
            println!("No bottles of beer on the wall.");
        }
    }
}

fn main() {
    let problem_number = 1u8;

    if problem_number == 1 {
        //p1
        for i in 0..=100 {
            match is_prime(i) {
                true => println!("{} is prime.", i),
                false => (),
            };
        }
    } else if problem_number == 2 {
        //p2
        for i in 0..=100 {
            for j in 0..=100 {
                match are_coprime((i, j)) {
                    true => println!("The numbers {} and {} are coprime.", i, j),
                    false => println!("The numbers {} and {} are not coprime.", i, j),
                };
            }
        }
    } else if problem_number == 3 {
        //p3
        bottles_of_beers();
    }
}
