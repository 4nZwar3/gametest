#[allow(dead_code)]
mod help;
mod player;
use {
    std::{
        collections::HashSet,
        io::{
            self,
            Write
        }
    },
    player::Player
};
fn player_add_hash(users: &mut HashSet<Player>) {
    loop {
        help::clear_screen();
        print!("Do you wish to create a new player? ");
        help::yesorno();
        io::stdout().flush().unwrap();
        let mut opcstr = String::new();
        io::stdin().read_line(&mut opcstr).unwrap();
        if let Ok('y') = opcstr.to_ascii_lowercase().trim().parse::<char>() {
            users.insert(Player::ask_new_usr());
        } else {
            help::clear_screen();
            println!("Ok. You can create another user later on by pressing Ctrl + N");
            let mut input = String::new();
            let _ = io::stdin().read_line(&mut input);
            help::clear_screen();
            break;
        }
    }
}
fn show_active_players(players: &HashSet<Player>) {
    println!("Current players.");
    for i in users {
        println!("{}", i.show_player());
        println!("");
    }
}
