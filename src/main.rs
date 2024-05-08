use std::{io::stdin, process::ExitCode};

use structures::structures::{Dictionnary, HangmanGame};

pub mod structures;

const MAX_ATTEMPT_NUMBER: i32 = 11;

fn main() -> ExitCode {
    // Charger le dictionnaire
    let dico = Dictionnary::load_from_file_path("./dico.txt".to_string());
    if dico.is_err() {
        println!("Une erreur est survenue : {}", dico.err().unwrap());
        return ExitCode::FAILURE;
    }

    let dico = dico.unwrap();
    // pour debug
    // dico.print();
    
    // choisir un mot dans le dictionnaire
    let word = dico.pick_random_word();
    
    let mut letters_found: usize = 0;
    let mut already_entered_chars = Vec::<char>::new();
    let mut attempt_number = 0;
    let mut hang_man_game = HangmanGame{word_to_found:word, found_letters: Vec::<char>::new() };

    // boucler tant que mot pas trouvé et nombre de tentative non atteint
    loop {
        let mut player_input = String::new();
        println!("Le mot -> [{}]",hang_man_game.get_obfuscated_word());
        println!("Saisir un caractère....({}/{})", attempt_number+1, MAX_ATTEMPT_NUMBER);
        let result = stdin().read_line(&mut player_input);
        if result.is_ok() {
            let letter = player_input.trim_end();
            if letter.len() != 1 {
               println!("Mauvaise saisie....! [{}]",letter); 
            }else{
               let entered_char = letter.chars().take(1).next().unwrap().to_lowercase().last().unwrap();
               // La lettre a été déjà saiie ?
               if !already_entered_chars.contains(&entered_char){
                    // La lettre est-elle contenue dans le mot ?
                    let number_found_in_word = word.count_letter(entered_char);
                    if number_found_in_word > 0 {
                        hang_man_game.found_letters.push(entered_char);
                    }else{
                        attempt_number += 1; // Pour le moment on est gentil, une lettre déjà saisie ne compte pas pour une nouvelle tentative
                    }

                    letters_found += number_found_in_word;
                    if letters_found == word.len(){
                        println!("Gagné !");
                        println!("Le mot -> [{}]",hang_man_game.get_obfuscated_word());
                        break;
                    }
                    
                    match number_found_in_word {
                        0 => println!("Aucune lettre...désolé :("),
                        _ => println!("La lettre apparaît {} fois", number_found_in_word)
                    }
                    // prise en compte de la lettre saisie trouvée ou non
                    already_entered_chars.push(entered_char);
                    
                    if attempt_number >= MAX_ATTEMPT_NUMBER {
                        println!("PERDU !");
                        println!("Le mot -> [{}]",hang_man_game.get_word());
                        break;
                    }
               
               }else{
                    println!("hummm...lettre déjà saisie...");  
               }
            }
        }
    }

    ExitCode::SUCCESS
}
