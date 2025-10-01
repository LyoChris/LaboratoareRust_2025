//"Sing" the 99 bottles of beer problem.

fn bottles_of_beers()
{
    let mut n = 99i32;
    while n > 0 {
        println!("{0} bottles of beer on the wall,\n{0} bottles of beer.\nTake one down, pass it around,", n);
        n -= 1;
        if n > 0 {
            println!("{} bottles of beer on the wall.\n", n);
        }
        else {
            println!("No bottles of beer on the wall.")
        }
    }
}

fn main() {
    bottles_of_beers();
}