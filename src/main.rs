/* DEVELOPED BY EZZEDDIN/EZZO
 *
 * A simulation of the popular indie game "Buckshot Roulette".
 * Original game made by Mike Klubnika
 *
 * Notes:
 * This version does not replicate the original AI's behavior but instead attempts to improve on
 * its decision making. It should be noted that the original dealer was made somewhat dumb in order
 * to balance the game. This simulation aims to create a more challenging dealer, but maintains
 * balance enough for it to be fair.
 * */


use std::io;
use rand::Rng;
use std::{thread::sleep, time::Duration, env};

#[derive(Debug, PartialEq)]
enum ItemType {
    CUFFS,
    MAGNIFIER,
    KNIFE,
    BEER,
    SMOKE,
    NONE
}

struct GameWatcher {
    running: bool,
    round_number: u16,
    round_state: bool,
    short_barrel: bool,
    player_is_cuffed: bool,
    dealer_is_cuffed: bool,
    player_turn: bool,
    shells: Vec<bool>,
    blank_count: u8,
    live_count: u8,
    player_health: u8,
    dealer_health: u8,
    player_items: Vec<ItemType>,
    dealer_items: Vec<ItemType>,
}

impl GameWatcher {
    fn new() -> GameWatcher {
        GameWatcher {
            running: true,
            round_number: 0,
            round_state: true,
            short_barrel: false,
            player_is_cuffed: false,
            dealer_is_cuffed: false,
            player_turn: true,
            shells: vec![],
            blank_count: 0,
            live_count: 0,
            player_health: 0,
            dealer_health: 0,
            player_items: vec![],
            dealer_items: vec![]
        }
    }
}

fn generate_item() -> ItemType {
    let item_type = rand::thread_rng().gen_range(0..5);
    match item_type {
        0 => { return ItemType::BEER;},
        1 => { return ItemType::CUFFS},
        2 => { return ItemType::KNIFE},
        3 => { return ItemType::SMOKE},
        4 => { return ItemType::MAGNIFIER},
        _ => { return ItemType::BEER}
    }
}
fn generate_shells(state: &mut GameWatcher) {
    while state.blank_count == 0 || state.live_count == 0 {
        state.shells.clear();
        state.blank_count = 0;
        state.live_count = 0;
        let shells = rand::thread_rng().gen_range(2..9);
        for shell in 0..shells {
            state.shells.push(rand::thread_rng().gen_bool(1.0 / 2.0));
            if state.shells[shell] == true {
                state.live_count += 1;
            }
            else {
                state.blank_count += 1;
            }
        }
    }
}

fn main() {
    println!("The dealer stares down at you with a creeping smile. The first round begins");
    env::set_var("RUST_BACKTRACE", "1");
    sleep(Duration::from_secs(2));
    let mut state: GameWatcher = GameWatcher::new(); 
    let health = rand::thread_rng().gen_range(2..7);
    state.player_health = health;
    state.dealer_health = health;
    while state.running {
        //beginnning of new round
        state.round_number += 1;
        if state.round_number > 1 {
            if state.player_items.len() >= 4 {
                println!("Your item slots are full.");
                sleep(Duration::from_secs(2));
            }
            else {
                println!("A slot from the table opens up, And from underneath, a box emerges up in it's place");
                sleep(Duration::from_secs(2));
                state.player_items.push(generate_item());
                println!("You have acquired a {:?}", state.player_items.last().unwrap());
                state.dealer_items.push(generate_item());
                sleep(Duration::from_secs(2));
            }
        }
        if check_health(&mut state) { break; }

        generate_shells(&mut state);
        println!("your health: {}", state.player_health);
        println!("dealer health: {}", state.dealer_health);
        sleep(Duration::from_secs(1));
        println!("{} shells loaded randomly. {} live and {} blank.", state.shells.len(), state.live_count, state.blank_count);
        sleep(Duration::from_secs(2));
        state.round_state = true;
        state.player_turn = true;
        while state.round_state == true {

            if check_health(&mut state) { break; }
            if state.shells.is_empty() {
                break;
            }
            if state.player_turn == true {

                if state.player_is_cuffed == true {
                    println!("You are cuffed. You get to play another turn");
                    state.player_is_cuffed = false;
                    state.player_turn = false;
                    break;
                }

                println!("What do you want to do?");
                println!("1- Shoot the dealer.");
                println!("2- Shoot yourself.");
                println!("3- Use item.");

                let mut input_text = String::new();
                io::stdin()
                    .read_line(&mut input_text)
                    .expect("failed to validate answer. Enter a number from the choices");
                let trimmed = input_text.trim();
                match trimmed.parse::<u8>() {
                    Ok(i) => { 
                        if i > 3 {
                            println!("choose a number within the range of choices");
                            continue;
                        }
                        execute_choice(i, &mut state);
                    },
                    Err(..) => println!("choose a number from the list of choices"),
                };
            }
            else {
                println!("dealer's turn");
                sleep(Duration::from_secs(2));
                execute_dealer(&mut state);
            }
        }
    }
    println!("Press any key to close the game.");
    let mut last_input = String::new();
    let _ = io::stdin().read_line(&mut last_input);
}

fn execute_choice(choice: u8, state: &mut GameWatcher) {
    match choice {
        1 => {
            println!("You point the gun at the dealer.");
            sleep(Duration::from_secs(2));
            let shell_pop = state.shells.pop();
            match shell_pop {
                Some(shell) => {
                    if shell == true {
                        let damage = if state.short_barrel == true {2} else {1};
                        state.live_count -= 1;
                        println!("It's live. You shoot the dealer for {}.", damage);
                        state.dealer_health -= damage;
                        sleep(Duration::from_secs(1));
                        println!("dealer has {} charges left.", state.dealer_health);
                        sleep(Duration::from_secs(2));
                    }
                    else {
                        state.blank_count -= 1;
                        println!("It's blank. You lose your turn.");
                        sleep(Duration::from_secs(2));
                    }
                    if state.short_barrel == true {
                        state.short_barrel = false;
                        println!("You hear a weird static noise, as you observe the shotgun's barrel fade in place");
                        sleep(Duration::from_secs(2));
                    }
                    state.player_turn = false;
                } 
                None => return
            }
        }
        2 => {
            println!("You point the gun at yourself");
            let shell_pop = state.shells.pop();
            sleep(Duration::from_secs(2));
            match shell_pop {
                Some(shell) => {
                    if shell == true {
                        state.live_count -= 1;
                        println!("It's live. You shoot yourself for 1.");
                        sleep(Duration::from_secs(1));
                        state.player_health -= 1;
                        println!("you have {} charges left.", state.player_health);
                        sleep(Duration::from_secs(2));
                        state.player_turn = false;
                    }
                    else {
                        state.blank_count -= 1;
                        println!("It's blank. You get to keep your turn.");
                        sleep(Duration::from_secs(2));
                    }
                } 
                None => return
            }
        }
        3 => {
            println!("which item would you like to use?");
            let mut beer_count = 0; 
            let mut smoke_count = 0; 
            let mut cuffs_count = 0; 
            let mut mag_count = 0; 
            let mut knife_count = 0;

            for item in 0..state.player_items.len() {
                match state.player_items[item] {
                    ItemType::BEER => beer_count += 1,
                    ItemType::CUFFS => cuffs_count += 1,
                    ItemType::KNIFE => knife_count += 1,
                    ItemType::SMOKE => smoke_count += 1,
                    ItemType::MAGNIFIER => mag_count += 1,
                    ItemType::NONE => continue
                }
            }
            println!("1-beer:x{}, racks the current shell out, you keep your turn.", beer_count);
            println!("2-smoke:x{}, gives you an extra charge.", smoke_count);
            println!("3-cuffs:x{}, skip the dealer's next turn.", cuffs_count);
            println!("4-magnifier:x{}, peek at the current shell in the chamber.", mag_count);
            println!("5-knife:x{}, shorten the barrel, deal double damage.", knife_count);
            let mut input_text = String::new();
            io::stdin()
                .read_line(&mut input_text)
                .expect("failed to validate answer. Enter a number from the choices");
            let trimmed = input_text.trim();
            match trimmed.parse::<i8>() {
                Ok(mut i) => {
                    while i > 5 || i < 1 {
                        println!("choose a valid number from the list");
                        String::clear(&mut input_text);
                        io::stdin()
                            .read_line(&mut input_text)
                            .expect("failed to validate answer. Enter a number from the choices");
                        i = input_text.trim().parse::<i8>().unwrap();
                    }
                        match i {
                            1 => if beer_count == 0 {println!("You don't have beer."); return;}
                            2 => if smoke_count == 0 {println!("You don't have smoke."); return;}
                            3 => if cuffs_count == 0 {println!("You don't have cuffs."); return;}
                            4 => if mag_count == 0 {println!("You don't have mag."); return;}
                            5 => if knife_count == 0 {println!("You don't have knife."); return;}
                            _ => {}
                        }
                    execute_item(state, i);
                }
                Err(..) => println!("choose a number from the list of choices"),
            }
        }
        _ => {
            return;
        }
    }
}

fn execute_dealer(state: &mut GameWatcher) {
    //TODO: Dealer item logic
    // - If you have smokes, use them right away
    // - Use beers when you don't know the current shell
    // - Always use cuffs
    // - Always use magnifiers
    // - Use knives when you know next shell is live

    if state.dealer_is_cuffed == true {
        println!("The dealer is cuffed. You get to play another turn");
        state.dealer_is_cuffed = false;
        state.player_turn = true;
        return;
    }
    let decision;
    if state.shells.len() == 1 {
        decision = state.shells[0];
    }
    else if state.live_count == 0 {
        decision = false;
    }
    else if state.blank_count == 0 {
        decision = true;
    }
    else {
        decision = rand::thread_rng().gen_bool(1.0 / 2.0);
    }

    if decision == true {
        //shoot player
        println!("The dealer points the gun at you.");
        sleep(Duration::from_secs(2));
        let shell_pop = state.shells.pop();
        match shell_pop {
            Some(shell) => {
                if shell == true {
                    state.live_count -= 1;
                    println!("It's live. Dealer shoots you for 1.");
                    state.player_health -= 1;
                    sleep(Duration::from_secs(2));
                    println!("you have {} charges left.", state.player_health);
                    sleep(Duration::from_secs(2));
                    state.player_turn = true;
                }
                else {
                    state.blank_count -= 1;
                    println!("It's blank. The dealer gives up his turn");
                    sleep(Duration::from_secs(2));
                    state.player_turn = true;
                    return;
                }
            } 
            None => return
        }
    }
    else {
        println!("The dealer points the gun at himself.");
        let shell_pop = state.shells.pop();
        sleep(Duration::from_secs(2));
        match shell_pop {
            Some(shell) => {
                if shell == true {
                    state.live_count -= 1;
                    println!("It's live. Dealer shoots shoots himself for 1.");
                    state.dealer_health -= 1;
                    sleep(Duration::from_secs(1));
                    println!("dealer has {} charges left.", state.dealer_health);
                    sleep(Duration::from_secs(2));
                    state.player_turn = true;
                }
                else {
                    state.blank_count -= 1;
                    println!("It's blank. The dealer retains his turn.");
                    sleep(Duration::from_secs(2));
                    return;
                }
            } 
            None => return
        }
    }
}

fn check_health(state: &mut GameWatcher) -> bool {
    if state.player_health == 0 {
        println!("You're out of charges. With nothing to heal you back, you are pronounced dead.");
        sleep(Duration::from_secs(2));
        state.round_state = false;
        state.running = false;
        return true;
    }
    if state.dealer_health == 0 {
        println!("The dealer's face disappears into the dark. Two red eyes glow in his place, as a briefcase is presented to you.");
        sleep(Duration::from_secs(2));
        println!("You open the briefcase to expose a wad of cash. You win.");
        sleep(Duration::from_secs(2));
        state.round_state = false;
        state.running = false;
        return true;
    }
    return false;
}


fn execute_item(state: &mut GameWatcher, item: i8) {
    //1- beer
    //2- smoke
    //3- cuffs
    //4- mag
    //5- knife

    let user = if state.player_turn { "You" } else { "The dealer"};
    let used_item;
    match item {
        1 => {
            used_item = ItemType::BEER;
            let shell_pop = state.shells.pop();
            let shell = shell_pop.unwrap();
            if shell == false {
                println!("{} used beer. Racked shell was a blank.", user); 
                sleep(Duration::from_secs(2));
            } else {
                println!("{} used beer. Racked shell was a live.", user);
                sleep(Duration::from_secs(2));
            }
        },
        2 => {
            used_item = ItemType::SMOKE;
            state.player_health += 1;
            println!("{} used smoke, {} gain an extra charge.", user, user);
            sleep(Duration::from_secs(2));
        },
        3 => {
            used_item = ItemType::CUFFS;
            state.dealer_is_cuffed = true;
            println!("{} used cuffs. Next turn will be skipped.", user);
            sleep(Duration::from_secs(2));
        },
        4 => {
            used_item = ItemType::MAGNIFIER;
            let shell_peek = state.shells.last();
            match shell_peek {
                Some(shell) => {
                    if user == "You" {
                        if *shell == false {
                            println!("You rack the shotgun halfway, and peek a BLANK shell."); 
                            sleep(Duration::from_secs(2));
                        }
                        else {println!("You rack the shotgun halfway, and peek a LIVE shell."); sleep(Duration::from_secs(2));}
                    }
                    else {
                        println!("The dealer breaks the magnifier and looks through it. 'Very interesting' he says."); 
                        sleep(Duration::from_secs(2));
                    }
                }
                None => return
            }
        },
        5 => {
            used_item = ItemType::KNIFE;
            if state.short_barrel {
                println!("the barrel is already cut.");
                sleep(Duration::from_secs(2));
            }
            state.short_barrel = true;
            println!("{} used knife. {} will deal double damage on the next live shot.", user, user);
            sleep(Duration::from_secs(2));
        },
        _ => used_item = ItemType::NONE,
    }

    if user == "You" {
        for i in 0..state.player_items.len() {
            if state.player_items.len() == 1 {
                state.player_items.remove(0);
                return;
            }
            if used_item == state.player_items[i] {
                state.player_items.remove(i); 
            }
        }
    }
    else {
        for i in 0..state.dealer_items.len() {
            if state.dealer_items.len() == 1 {
                state.player_items.remove(0);
                return;
            }
            if used_item == state.dealer_items[i] {
                state.player_items.remove(i); 
            }
        }
    }
}
