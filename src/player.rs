use {
    std::{
        io::{
            self, 
            Write
        }
    },
    crate::help
};
#[derive(PartialEq, Eq, Hash)]
pub enum Race {
    HUMAN,
    ELF,
    ORC,
}
#[derive(PartialEq, Eq, Hash)]
pub struct Player {
    pub name: String,
    pub race: Race,
    password: String,
}
impl Player {
    fn ask_new_nam() -> String {
        let mut nam = String::new();
        help::read_something("name", &mut nam);
        nam
    }
    fn ask_new_rac() -> Race {
        loop {
            help::clear_screen();
            print!("Enter your character's race\n1)\tHuman\n2)\tElf\n3)\tOrc\nSelect your option: ");
            io::stdout().flush().expect("Error while flushing race menu.");
            let mut racstr = String::new();
            io::stdin().read_line(&mut racstr).expect("Failed reading race from user.");
            match racstr.trim().parse::<i32>() {
                Ok(num) => match num {
                    1 => return Race::HUMAN,
                    2 => return Race::ELF,
                    3 => return Race::ORC,
                    _ => {
                        help::clear_screen();
                        println!("NOT A RACE!!");
                        help::get_char();
                        continue;
                    }
                },
                Err(_) => {
                    help::clear_screen();
                    println!("Insert the number at the left of the option...");
                    help::get_char();
                    continue;
                }
            }
        }
    }
    fn ask_new_pas() -> String {
        loop {
            let mut pas = String::new();
            let mut conf = String::new();
            help::read_something("password", &mut pas);
            help::read_something("password confirmation", &mut conf);
            help::clear_screen();
            if !pas.eq(&conf) {
                println!("Passwords don't match, try again.");
                help::get_char();
                continue;
            }
            print!("Are you sure you want to set your password to {}? ", pas.trim());
            help::yesorno();
            io::stdout().flush().expect("Error while flushing on password confirmation");
            if !help::answer() {
                continue;
            }
            return pas.trim().to_string();
        }
    }
    pub fn from(nam: String, rac: Race, pas: String) -> Player {
        Player {
            name: nam,
            race: rac,
            password: pas
        }
    }
    pub fn ask_new_plyr() -> Player {
        loop {
            let result = Player::from(Self::ask_new_nam(), Self::ask_new_rac(), Self::ask_new_pas());
            help::clear_screen();
            print!("Here's the gathered data.\n{}\nIs this ok? ", result.show_player());
            help::yesorno();
            io::stdout().flush().expect("Failed to flush after asking if the player received was correct.");
            if help::answer() {
                return result;
            }
        }
    }
    pub fn show_race(&self) -> String {
        String::from(match self.race {
            Race::HUMAN => "Human",
            Race::ELF => "Elf",
            Race::ORC => "Orc",
        })
    }
    pub fn show_player(&self) -> String {
        String::from("Name: ".to_owned() + self.name.as_str() + "\nRace: " + self.show_race().as_str())
    }
}
