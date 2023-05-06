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
    player::Player,
};
fn player_add_hash(users: &mut HashSet<Player>) {
    loop {
        help::clear_screen();
        print!("Do you wish to create a new player? ");
        help::yesorno();
        io::stdout().flush().unwrap();
        if help::answer() {
            users.insert(Player::ask_new_plyr());
        } else {
            help::clear_screen();
            println!("Ok. You can create another user later on by pressing Ctrl + N");
            help::get_char();
            help::clear_screen();
            break;
        }
    }
}
fn show_active_players(players: &HashSet<Player>) {
    if players.is_empty() {
        return;
    }
    println!("Current players.");
    for player in players.iter() {
        println!("\n{}\nRace: {}", player.name, player.show_race());
    }
    print!("\n");
}
fn main() {
    let mut players: HashSet<Player> = HashSet::new();
    player_add_hash(&mut players);
    show_active_players(&players);
}
