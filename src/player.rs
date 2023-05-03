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
    NonExistent
}
#[derive(PartialEq, Eq, Hash)]
pub struct Player {
    pub name: String,
    pub race: Race,
    //password: String,
}
impl Player {
    pub fn ask_new_plyr() -> Player {
        loop {
            let mut nam = String::new();
            let mut rac = Race::NonExistent;
            help::clear_screen();
            print!("Insert your character's name: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut nam).unwrap();
            nam = nam.trim().to_string();
            while rac == Race::NonExistent {
                help::clear_screen();
                print!("Enter your character's race\n1)\tHuman\n2)\tElf\n3)\tOrc\nSelect your option: ");
                io::stdout().flush().unwrap();
                let mut racstr = String::new();
                io::stdin().read_line(&mut racstr).unwrap();
                match racstr.trim().parse::<i32>() {
                    Ok(num) => match num {
                        1 => rac = Race::HUMAN,
                        2 => rac = Race::ELF,
                        3 => rac = Race::ORC,
                        _ => {
                            help::clear_screen();
                            println!("NOT A RACE!!");
                            continue;
                        }
                    },
                    Err(_) => {
                        help::clear_screen();
                        println!("Insert the number at the left of the option...");
                    }
                }
            }
            help::clear_screen();
            let result = Player {
                name: nam,
                race: rac
            };
            print!("Here's the gathered data.\n{}\nIs this ok? ", result.show_player());
            help::yesorno();
            io::stdout().flush().unwrap();
            let mut opcstr = String::new();
            io::stdin().read_line(&mut opcstr).unwrap();
            if let Ok('y') = opcstr.to_ascii_lowercase().trim().parse::<char>() {
                return result;
            }
        }
    }
    pub fn show_player(&self) -> String {
        let mut ret = String::new();
        ret.push_str("Name: ");
        ret.push_str(self.name.as_str());
        ret.push_str("\nRace: ");
        ret.push_str(match self.race {
            Race::HUMAN => "Human",
            Race::ELF => "Elf",
            Race::ORC => "Orc",
            Race::NonExistent => "Literally nothing",
        });
        ret
    }
}
