use rand::Rng;
use rusqlite::Connection;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};

trait MyCommand {
    fn get_name(&self) -> &str {
        "Default name"
    }
    fn exec(&mut self, args: &[&str]);
}

fn barbut_roll_generator() -> (i32, i32) {
    let mut rng = rand::thread_rng();
    (rng.gen_range(1..=6), rng.gen_range(1..=6))
}

fn levensthein_distance(s1: &str, s2: &str, m: usize, n: usize) -> usize {
    if m == 0 {
        return n;
    }

    if n == 0 {
        return m;
    }

    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();

    if s1_chars[m - 1] == s2_chars[n - 1] {
        return levensthein_distance(s1, s2, m - 1, n - 1);
    }

    1 + levensthein_distance(s1, s2, m, n - 1)
        .min(levensthein_distance(s1, s2, m - 1, n))
        .min(levensthein_distance(s1, s2, m - 1, n - 1))
}

struct PingCommand {}
struct CountCommand {}
struct TimesCommand {
    count: i32,
}
struct BarbutCommand {}
struct StopCommand {}
struct BookmarkCommand {}
struct Bookmark {
    name: String,
    url: String,
}
struct Terminal {
    commands: Vec<Box<dyn MyCommand>>,
}

impl MyCommand for PingCommand {
    fn get_name(&self) -> &str {
        "ping"
    }
    fn exec(&mut self, _args: &[&str]) {
        println!("pong");
    }
}

impl MyCommand for CountCommand {
    fn get_name(&self) -> &str {
        "count"
    }
    fn exec(&mut self, args: &[&str]) {
        let mut i = 0u8;
        for _ in args {
            i += 1;
        }

        println!("counted {i} args");
    }
}

impl MyCommand for TimesCommand {
    fn get_name(&self) -> &str {
        "times"
    }
    fn exec(&mut self, _args: &[&str]) {
        self.count += 1;
        println!("Times called {} times.", self.count.clone());
    }
}

impl MyCommand for BarbutCommand {
    fn get_name(&self) -> &str {
        "barbut"
    }
    fn exec(&mut self, _args: &[&str]) {
        let player: (i32, i32) = barbut_roll_generator();
        let npc: (i32, i32) = barbut_roll_generator();
        println!("-------------------------------------------");
        println!(
            "You roll the dice... \n You got {0} and {1}",
            player.0, player.1
        );
        println!("NPC rolls the dice... \n He got {0} and {1}", npc.0, npc.1);
        match (player.0 + player.1).cmp(&(npc.0 + npc.1)) {
            Ordering::Greater => println!("You got more than the npc, you WIN!"),
            Ordering::Equal => println!("You got the exact score as the npc, it's a TIE!"),
            Ordering::Less => println!("You got less than the npc, it's a LOSS"),
        };
        println!("-------------------------------------------\n");
    }
}

impl MyCommand for StopCommand {
    fn get_name(&self) -> &str {
        "stop"
    }
    fn exec(&mut self, _args: &[&str]) {
        std::process::exit(0);
    }
}

impl MyCommand for BookmarkCommand {
    fn get_name(&self) -> &str {
        "bk"
    }
    fn exec(&mut self, arg: &[&str]) {
        let conn = match Connection::open("bookmarks.db") {
            Ok(c) => c,
            Err(e) => {
                println! {"Couldn't connect, {e}"};
                return;
            }
        };
        let create = r"
            create table if not exists bookmarks (
                name text    not null,
                url  text not null
                );
            ";
        match conn.execute(create, ()) {
            Ok(_c) => (),
            Err(e) => {
                println!("Couldn't execute, {e}");
                return;
            }
        };

        let word = arg.first();
        let key = match word {
            Some(w) => w,
            None => return,
        };

        if *key == "add" {
            let mut n = arg.get(1);
            let name = match n {
                Some(w) => w,
                None => return,
            };
            n = arg.get(2);
            let url = match n {
                Some(w) => w,
                None => return,
            };
            match conn.execute(
                "insert into bookmarks (name, url) values (?1, ?2);",
                (name, url),
            ) {
                Ok(_c) => (),
                Err(e) => {
                    println!("Couldn't execute, {e}");
                    return;
                }
            };
        }

        if *key == "search" {
            let n = arg.get(1);
            let search = match n {
                Some(w) => w,
                None => return,
            };
            let sql = format!("SELECT * FROM bookmarks WHERE name LIKE '%{search}%'");

            let mut stmt = match conn.prepare(&sql) {
                Ok(s) => s,
                Err(e) => {
                    println!("Couldn't prepare statement: {e}");
                    return;
                }
            };

            let person_iter = match stmt.query_map([], |row| {
                let name: String = match row.get("name") {
                    Ok(v) => v,
                    Err(e) => {
                        println!("Failed to get name: {e}");
                        return Ok(Bookmark {
                            name: "".to_string(),
                            url: "".to_string(),
                        }); // skip this row
                    }
                };
                let url: String = match row.get("url") {
                    Ok(v) => v,
                    Err(e) => {
                        println!("Failed to get url: {e}");
                        return Ok(Bookmark {
                            name: "".to_string(),
                            url: "".to_string(),
                        });
                    }
                };
                Ok(Bookmark { name, url })
            }) {
                Ok(iter) => iter,
                Err(e) => {
                    println!("Query failed: {e}");
                    return;
                }
            };

            for item in person_iter {
                match item {
                    Ok(b) => println!("name={}, url={}", b.name, b.url),
                    Err(e) => println!("Error reading row: {e}"),
                }
            }
        }
    }
}

impl Terminal {
    fn new() -> Self {
        let commands: Vec<Box<dyn MyCommand>> = vec![Box::new(StopCommand {})];

        Terminal { commands }
    }

    fn register(&mut self, arg: Box<dyn MyCommand>) {
        self.commands.push(arg);
    }

    fn suggestions(&self, s: &str) {
        let s_lower = s.to_lowercase();
        let mut min = usize::MAX;
        let mut suggestion = " ";
        for registered in &self.commands {
            let dist = levensthein_distance(
                registered.get_name(),
                &s_lower,
                registered.get_name().chars().count(),
                s_lower.chars().count(),
            );
            if dist < min {
                min = dist;
                suggestion = registered.get_name();
            }
        }

        println!("'{s}' is not a valid function. Did you mean to write '{suggestion}'?");
    }

    fn run(&mut self) {
        let file = match File::open("commands.txt") {
            Ok(exel) => exel,
            Err(e) => {
                println!("Couldn't open file, error: {e}");
                return;
            }
        };

        let reader = BufReader::new(file);

        for lines in reader.lines() {
            let line = match lines {
                Ok(text) => text,
                Err(e) => {
                    println!("Couldn't read line, error {e}");
                    continue;
                }
            };

            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let mut words = line.split_whitespace();
            let command_name = match words.next() {
                Some(name) => name,
                None => continue,
            };
            let args: Vec<&str> = words.collect();

            let mut wrong: bool = true;
            for registered in &mut self.commands {
                if command_name == registered.get_name() {
                    registered.exec(&args);
                    wrong = false;
                }
            }

            if wrong {
                self.suggestions(command_name);
            }
        }
    }
}

fn main() {
    let mut terminal = Terminal::new();

    terminal.register(Box::new(PingCommand {}));
    terminal.register(Box::new(CountCommand {}));
    terminal.register(Box::new(TimesCommand { count: 0 }));
    terminal.register(Box::new(BarbutCommand {}));
    terminal.register(Box::new(BookmarkCommand {}));

    terminal.run();
}
