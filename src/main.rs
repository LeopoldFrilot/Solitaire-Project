const CARD_SUITS: [&str; 4] = ["H", "C", "D", "S"];
const CARD_VALUES: [&str; 13] = [" A", " 2", " 3", " 4", " 5", " 6", " 7", " 8", " 9", "10", " J", " Q", " K"];
const NULL_STRING: &str = "NULL";
const NULL_USIZE: usize = 99;
const TABLEAU_PILE_COUNT: usize = 7;
const DECK_EMPTY_STRING: &str = " N/A";
const CARD_SLOT_EMPTY_STRING: &str = "    ";
const FACE_DOWN_CARD_STRING: &str = " \u{025AF}  ";
const WASTE_SELECTED_USIZE: usize = 44;

use rand::thread_rng;
use rand::seq::SliceRandom;
use std::io;
use itertools::iproduct;

fn main() {
    start_solitaire();
}

fn start_solitaire()
{
    let mut waste: Vec<(String, usize, usize, bool)> = Vec::new();
    let mut tableau: Vec<Vec<(String, usize, usize, bool)>> = Vec::new();
    let mut foundation: Vec<Vec<(String, usize, usize, bool)>> = Vec::new();
    let mut deck = create_deck();
    shuffle_deck(&mut deck);
    set_up_tableau(&mut deck, &mut tableau);
    set_up_foundation(&mut foundation);
    play_solitaire(&mut deck, &mut waste, &mut tableau, &mut foundation)
}

fn get_top_of_pile_card(pile: Vec<(String, usize, usize, bool)>) -> (String, usize, usize, bool) {
    match pile.last()
    {
        Some(card) => {
            card.to_owned()
        }
        None => (NULL_STRING.to_string(), NULL_USIZE, NULL_USIZE, false),
    }
}

fn play_solitaire(deck: &mut Vec<(String, usize, usize, bool)>, 
                waste: &mut Vec<(String, usize, usize, bool)>, 
                tableau: &mut Vec<Vec<(String, usize, usize, bool)>>, 
                foundation: &mut Vec<Vec<(String, usize, usize, bool)>>) {
    let mut tableau_selection = NULL_USIZE;
    
    'main_loop: loop {
        display_title();
        display_board(deck.to_owned(), waste.to_owned(), tableau.to_owned(), foundation.to_owned());
        display_status(&mut tableau_selection);
        display_instructions();

        if deck.is_empty() && waste.is_empty() && {
            let mut empty = true;
            'tableau_empty_check: for pile in tableau.to_owned() {
                if !pile.is_empty() {
                    empty = false;
                    break 'tableau_empty_check;
                }
            }
            empty
        }{
            println!("YOU'VE WON!");
            break 'main_loop;
        }

        let mut player_choice =  String::new();

        io::stdin()
            .read_line(&mut player_choice)
            .expect("ERROR: Failed to read player input correctly");
        
        // Handle string input
        player_choice = player_choice.trim().to_lowercase();

        if player_choice == "q" {
            break 'main_loop;
        } else if player_choice == "h" {
            try_to_place_in_foundation_slot_from_selection(0, foundation, tableau, &mut tableau_selection, waste);
        } else if player_choice == "c" {
            try_to_place_in_foundation_slot_from_selection(1, foundation, tableau, &mut tableau_selection, waste);
        } else if player_choice == "d" {
            try_to_place_in_foundation_slot_from_selection(2, foundation, tableau, &mut tableau_selection, waste);
        } else if player_choice == "s" {
            try_to_place_in_foundation_slot_from_selection(3, foundation, tableau, &mut tableau_selection, waste);
        } else if player_choice == "draw" {
            draw_three_from_stock(deck, waste);
        } else if player_choice == "waste" {
            on_waste_selection(&mut tableau_selection);
        } else {
            // Handle int input
            match player_choice.parse::<usize>() {
                Ok(num) => {
                    on_tableau_selection(tableau, num - 1, &mut tableau_selection, waste)
                },
                Err(_) => continue 'main_loop,
            }
        }
    }
}

fn draw_three_from_stock(deck: &mut Vec<(String, usize, usize, bool)>, 
                        waste: &mut Vec<(String, usize, usize, bool)>) {
    'triple_draw: for _ in 0..3 {
        if deck.len() > 0 {
            waste.push(draw_card_from_pile(deck));
        } else if waste.len() > 0 {
            deck.append(waste);
            waste.push(draw_card_from_pile(deck));
        } else {
            break 'triple_draw;
        }
    }
}

fn on_waste_selection(tableau_selection: &mut usize) {
    if *tableau_selection == WASTE_SELECTED_USIZE {
        *tableau_selection = NULL_USIZE;
    } else {
        *tableau_selection = WASTE_SELECTED_USIZE;
    }
}

fn try_to_place_in_foundation_slot_from_selection(slot: usize, foundation: &mut Vec<Vec<(String, usize, usize, bool)>>, 
                                                tableau: &mut Vec<Vec<(String, usize, usize, bool)>>, 
                                                tableau_selection: &mut usize, 
                                                waste: &mut Vec<(String, usize, usize, bool)>) {
    if *tableau_selection != NULL_USIZE {
        if *tableau_selection == WASTE_SELECTED_USIZE { 
            if try_to_place_in_foundation_slot(slot, foundation, waste) {
                *tableau_selection = NULL_USIZE;
            }
        } else {
            let source_pile = tableau.get_mut(*tableau_selection).expect("ERROR: Pile index breach");
            if try_to_place_in_foundation_slot(slot, foundation, source_pile) {
                *tableau_selection = NULL_USIZE;
                flip_top_card_in_pile(source_pile);
            }
        }
    }
}

fn try_to_place_in_foundation_slot(slot: usize, foundation: &mut Vec<Vec<(String, usize, usize, bool)>>, source_pile: &mut Vec<(String, usize, usize, bool)>) -> bool {
    if slot < foundation.len() {
        let foundation_slot = foundation.get_mut(slot).expect("ERROR: Pile index breach");
        let foundation_slot_length = foundation_slot.len();
        let foundation_slot_top_value = get_top_of_pile_card(foundation_slot.to_vec()).2;
        let source_top_value = get_top_of_pile_card(source_pile.to_vec()).2;

        if (source_top_value == 1 && foundation_slot_length == 0) || (source_top_value > foundation_slot_top_value && source_top_value - foundation_slot_top_value == 1) {
            foundation_slot.push(draw_card_from_pile(source_pile));
            true
        } else {
            false
        }
    } else {
        false
    }
}

fn on_tableau_selection(tableau: &mut Vec<Vec<(String, usize, usize, bool)>>, 
                        pile_index: usize, 
                        tableau_selection: &mut usize, 
                        waste: &mut Vec<(String, usize, usize, bool)>,) {
    if pile_index < tableau.len() {
        if *tableau_selection == NULL_USIZE {
            *tableau_selection = pile_index;
        } else if *tableau_selection == pile_index{
            *tableau_selection = NULL_USIZE;
        } else {
            let target_pile_copy = tableau.get(pile_index).expect("ERROR: Pile index breach");
            if *tableau_selection == WASTE_SELECTED_USIZE {
                let mut source_pile: Vec<(String, usize, usize, bool)> = Vec::new();
                source_pile.push(get_top_of_pile_card(waste.to_vec()));
                if pile_can_be_placed(source_pile.to_vec(), target_pile_copy.to_vec()) {
                    draw_card_from_pile(waste);
                    let mut target_pile = tableau.get_mut(pile_index).expect("ERROR: Pile index breach");
                    place_pile_on_pile(source_pile, &mut target_pile);
                    *tableau_selection = NULL_USIZE;
                    flip_top_card_in_pile(target_pile);
                }
            } else {
                let source_pile_copy = tableau.get(*tableau_selection).expect("ERROR: Pile index breach");
                if pile_can_be_placed(get_flipped_pile(source_pile_copy.to_vec()), target_pile_copy.to_vec()) {
                    let mut source_pile = tableau.get_mut(*tableau_selection).expect("ERROR: Pile index breach");
                    let pile = draw_flipped_pile(&mut source_pile);
                    let mut target_pile = tableau.get_mut(pile_index).expect("ERROR: Pile index breach");
                    place_pile_on_pile(pile, &mut target_pile);
                }
    
                let source_pile = tableau.get_mut(*tableau_selection).expect("ERROR: Pile index breach");
                *tableau_selection = NULL_USIZE;
                flip_top_card_in_pile(source_pile);
            }
        }
    }
}

fn flip_top_card_in_pile(pile: &mut Vec<(String, usize, usize, bool)>) {
    if pile.len() > 0 {
        match pile.last_mut() {
            Some(card) => card.3 = true,
            None => return,
        }
    }
}

fn place_pile_on_pile(source_pile: Vec<(String, usize, usize, bool)>, target_pile: &mut Vec<(String, usize, usize, bool)>) {
    for card in source_pile {
        target_pile.push(card);
    }
}

fn pile_can_be_placed(source_pile: Vec<(String, usize, usize, bool)>, target_pile: Vec<(String, usize, usize, bool)>) -> bool {
    let first_card = match source_pile.first() {
        Some(card) => card.to_owned(),
        None => { return false },
    };

    if first_card.2 == 13 && target_pile.len() == 0 {
        true
    } else if target_pile.len() > 0 {
        let target_pile_card = get_top_of_pile_card(target_pile);
        if is_different_color_suit(first_card.1, target_pile_card.1) && target_pile_card.2 > first_card.2 && target_pile_card.2 - first_card.2 == 1 {
            true
        } else {
            false
        }
    }
    else {
        false
    }
}

fn is_different_color_suit(suit_a: usize, suit_b: usize) -> bool {
    if suit_a % 2 == 0 && suit_b % 2 == 0 { // both red
        false
    } else if suit_a % 2 != 0 && suit_b % 2 != 0 { // both black
        false
    }
    else {
        true
    }
}

fn get_flipped_pile(pile: Vec<(String, usize, usize, bool)>) -> Vec<(String, usize, usize, bool)> {
    let mut sub_pile: Vec<(String, usize, usize, bool)> = Vec::new();
    for card in pile {
        if card.3 == true {
            sub_pile.push(card);
        }
    }
    sub_pile
}

fn draw_flipped_pile(pile: &mut Vec<(String, usize, usize, bool)>) -> Vec<(String, usize, usize, bool)> {
    let mut sub_pile: Vec<(String, usize, usize, bool)> = Vec::new();
    let mut finished = false;
    while !finished {
        finished = match pile.last() {
            Some(card) => {
                if card.3 == true {
                    sub_pile.insert(0, draw_card_from_pile(pile));
                    false
                } else {
                    true
                }
            },
            None => true,
        }
    }
    sub_pile
}

fn draw_card_from_pile(pile: &mut Vec<(String, usize, usize, bool)>) -> (String, usize, usize, bool) {
    match pile.pop() {
        Some(card) => card,
        None => (NULL_STRING.to_string(), NULL_USIZE, NULL_USIZE, false),
    }
}

fn set_up_foundation(foundation:&mut Vec<Vec<(String, usize, usize, bool)>>) {
    foundation.clear();
    for _ in 0..CARD_SUITS.len() {
        let new_pile: Vec<(String, usize, usize, bool)> = Vec::new();
        foundation.push(new_pile);
    }
}

fn set_up_tableau(deck: &mut Vec<(String, usize, usize, bool)>, tableau:&mut Vec<Vec<(String, usize, usize, bool)>>) {
    tableau.clear();
    for pile_index in 0..TABLEAU_PILE_COUNT {
        let mut new_pile: Vec<(String, usize, usize, bool)> = Vec::new();
        for _ in 0..(pile_index + 1) {
            new_pile.push(draw_card_from_pile(deck));
        }
        new_pile.last_mut().expect("ERROR: Pile has no cards in it").3 = true;
        tableau.push(new_pile);
    }
}

fn display_instructions() {
    println!("Type 'q' to quit");
    println!("Type 'draw' to draw from the deck");
    println!("Type 'waste' to select the waste pile");
    println!("Type a number (1-7) to select a pile");
    println!("Type the number again to deslect it");
    println!("Type 'h', 'c', 'd', or 's' to move first card in selected pile to the foundation");
}

fn display_status(tableau_selection: &mut usize) {
    let selected_pile = if *tableau_selection == NULL_USIZE {
        "None".to_string()
    } else if *tableau_selection == WASTE_SELECTED_USIZE{
        "Waste".to_string()
    } else {
        (*tableau_selection + 1).to_string()
    };
    println!("Selected pile: {selected_pile}");
}

fn display_title() {
    println!("Playing Solitaire!");
}

fn display_board(deck: Vec<(String, usize, usize, bool)>, 
            waste: Vec<(String, usize, usize, bool)>, 
            tableau: Vec<Vec<(String, usize, usize, bool)>>, 
            foundation: Vec<Vec<(String, usize, usize, bool)>>) {
    let horiz_buffer = "  ";
    let vert_buffer = "\n";
    let mut full_string = String::new();
    let mut card_name: String;

    // top top line
    full_string = format!("{full_string}Deck{horiz_buffer}Waste             Foundation{vert_buffer}");
    full_string = format!("{vert_buffer}{full_string}                    H {horiz_buffer}  C {horiz_buffer}  D {horiz_buffer}  S{vert_buffer}");


    // top line
    if deck.len() > 0 {
        card_name = FACE_DOWN_CARD_STRING.to_string();
    } else {
        card_name = DECK_EMPTY_STRING.to_string();
    }
    full_string = format!("{full_string}{card_name}{horiz_buffer}");

    card_name = get_top_of_pile_name(waste);
    full_string = format!("{full_string}{card_name}{horiz_buffer}{CARD_SLOT_EMPTY_STRING}{horiz_buffer}");
    for pile in foundation {
        card_name = get_top_of_pile_name(pile);
        full_string = format!("{full_string}{card_name}{horiz_buffer}");
    }
    full_string = format!("{full_string}{vert_buffer}{vert_buffer}");

    // tableau
    let mut found = true;
    let mut row = 0;
    while found {
        found = false;
        for pile in &tableau {
            match pile.get(row) {
                Some(card) => {
                    card_name = card.0.to_string();
                    if card.3 == true {
                        full_string = format!("{full_string}{card_name}");
                    } else {
                        full_string = format!("{full_string}{FACE_DOWN_CARD_STRING}");
                    }
                    
                    found = true;
                },
                None => {
                    if row == 0 {
                        full_string = format!("{full_string}{DECK_EMPTY_STRING}");
                    } else {
                        full_string = format!("{full_string}{CARD_SLOT_EMPTY_STRING}");
                    }
                },
            }
            full_string = format!("{full_string}{horiz_buffer}");
        }
        full_string = format!("{full_string}{vert_buffer}");
        row += 1;
    }
    print!("{full_string}");
}

fn get_top_of_pile_name(pile: Vec<(String, usize, usize, bool)>) -> String {
    match pile.last()
    {
        Some(card) => card.0.to_string(),
        None => DECK_EMPTY_STRING.to_string(),
    }
}

fn suit_name_to_int(suit: &str) -> usize {
    let mut x = NULL_USIZE;
    'suit_check: for index in 0..CARD_SUITS.len() {
        if CARD_SUITS[index] == suit {
            x = index;
            break 'suit_check;
        }
    }
    x
}

fn card_value_to_int(value: &str) -> usize {
    let value = value.trim();
    let x = match value.parse() {
        Ok(num) => num,
        Err(_) => {
            if value == "A" {
                1
            } else if value == "J" {
                11
            } else if value == "Q" {
                12
            } else if value == "K" {
                13
            }
            else {
                NULL_USIZE
            }
        }
    };
    x
}

fn shuffle_deck(deck: &mut Vec<(String, usize, usize, bool)>) {
    deck.shuffle(&mut thread_rng());
}

fn create_deck() -> Vec<(String, usize, usize, bool)> {
    let deck = iproduct!(CARD_SUITS.clone(), CARD_VALUES.clone()).map(|(suit, value)| (format!("{value}|{suit}"), suit_name_to_int(suit), card_value_to_int(value), false)).collect();
    deck
}