use std::io;
use rand::Rng;
use std::thread::sleep;
use std::time::Duration;

struct GameWatcher {
    running: bool,
    round_state: bool,
    my_turn: bool,
    shells: Vec<bool>,
    player_health: u8,
    player_cuffs: u8,
    player_beer: u8,
    player_smoke: u8,
    player_knife: u8,
    player_magnifier: u8,
    dealer_health: u8,
    dealer_cuffs: u8,
    dealer_beer: u8,
    dealer_smoke: u8,
    dealer_knife: u8,
    dealer_magnifier: u8,
}

impl GameWatcher {
    fn new() -> GameWatcher {
        GameWatcher {
            running: true,
            round_state: true,
            my_turn: true,
            shells: vec![],
            player_health: 0,
            player_cuffs: 0,
            player_beer: 0,
            player_smoke: 0,
            player_knife: 0,
            player_magnifier: 0,
            dealer_health: 0,
            dealer_cuffs: 0,
            dealer_beer: 0,
            dealer_smoke: 0,
            dealer_knife: 0,
            dealer_magnifier: 0,
        }
    }
}

fn main() {
    println!("After intense mental preparation, you finally set your mind to it.\nYou're gonna play. Because that's just who you are.");
    sleep(Duration::from_secs(2));
    let mut state: GameWatcher = GameWatcher::new(); 
    let health = rand::thread_rng().gen_range(2..7);
    state.player_health = health;
    state.dealer_health = health;
    while state.running {
        //beginnning of new round
        let shells = rand::thread_rng().gen_range(2..9);
        let mut blank_count = 0;
        let mut live_count = 0;
        for shell in 0..shells {
            state.shells.push(rand::thread_rng().gen_bool(1.0 / 2.0));
            if state.shells[shell] == true {
                live_count += 1;
            }
            else {
                blank_count += 1;
            }
        }
        if live_count == 0 || blank_count == 0 {
            state.shells.clear();
        }
        else {
            println!("your health: {}", state.player_health);
            println!("dealer health: {}", state.dealer_health);
            sleep(Duration::from_secs(1));
            println!("{} shells loaded randomly. {} live and {} blank.", state.shells.len(), live_count, blank_count);
            sleep(Duration::from_secs(2));
        }
        state.round_state = true;
        state.my_turn = true;
        while state.round_state == true {

            if state.shells.is_empty() {
                state.round_state = false;
                break;
            }
            if state.player_health == 0 {
                println!("You're out of charges. With nothing to heal you back, you are pronounced dead.");
                state.round_state = false;
                state.running = false;
            }
            if state.dealer_health == 0 {
                println!("The dealer's face disappears into the dark. Two red eyes glow in his place, as a briefcase is presented to you.");
                println!("You open the briefcase to expose a wad of cash. You win.");
                state.round_state = false;
                state.running = false;
                break;
            }
            if state.my_turn == true {
                println!("What do you want to do?");
                println!("1- Shoot the dealer.");
                println!("2- Shoot yourself.");
                //        println!("3- Use item.");

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
                        println!("It's live. You shoot the dealer for 1.");
                        state.dealer_health -= 1;
                        sleep(Duration::from_secs(1));
                        println!("dealer has {} charges left.", state.dealer_health);
                    }
                    else {
                        println!("It's blank. You lose your turn.");
                        sleep(Duration::from_secs(2));
                    }
                    state.my_turn = false;
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
                        println!("It's live. You shoot yourself for 1.");
                        sleep(Duration::from_secs(1));
                        state.player_health -= 1;
                        println!("you have {} charges left.", state.player_health);
                        state.my_turn = false;
                    }
                    else {
                        println!("It's blank. You get to keep your turn.");
                    }
                } 
                None => return
            }
        }
        _ => {
            return;
        }
    }
}

fn execute_dealer(state: &mut GameWatcher) {
    let decision;
    if state.shells.len() == 1 {
        decision = state.shells[0];
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
                    println!("It's live. Dealer shoots you for 1.");
                    state.player_health -= 1;
                    sleep(Duration::from_secs(2));
                    println!("you have {} charges left.", state.player_health);
                    state.my_turn = true;
                }
                else {
                    println!("It's blank. The dealer gives up his turn");
                    sleep(Duration::from_secs(2));
                    state.my_turn = true;
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
                    println!("It's live. Dealer shoots shoots himself for 1.");
                    state.dealer_health -= 1;
                    sleep(Duration::from_secs(1));
                    println!("dealer has {} charges left.", state.dealer_health);
                    state.my_turn = true;
                }
                else {
                    println!("It's blank. The dealer retains his turn.");
                    return;
                }
            } 
            None => return
        }
    }
}
