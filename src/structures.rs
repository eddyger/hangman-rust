use std::{fmt::Display, fs, io::{BufRead, BufReader, Error}};

use rand::Rng;


pub struct Dictionnary {
    pub liste:Vec<Word>,
    pub min_words_length:u8,
}

impl Dictionnary {
    pub fn load_from_file_path(&mut self, file_path:String) -> Result<(), Error> {
        println!("Chargement du fichier : [{}] ...", file_path);
        // Open the file and handle the result with the ? operator
        let file = fs::File::open(file_path)?;

        // fill the dictionnary
        let reader = BufReader::new(file);
        let mut loaded_words = 0;
        for line in reader.lines() {
            let line = line?;
            if line.len() >= self.min_words_length.into() {
                self.liste.push(Word(line.to_string())); // TODO transformer les caractères accentués en leur équivalent
                loaded_words += 1;
                // println!("Add {} , {} car",line.to_string(), line.len());
            }
        }

        println!("Nombre de mots chargés: [{}]", loaded_words);

        Ok(())
        
    }

    pub fn new() -> Self {
            // Create a new instance of Dictionnary with an empty vector
        let dictionnary = Dictionnary {
            liste: Default::default(),
            min_words_length: 5
        };

        dictionnary
    }

    pub fn with_min_words_length(&mut self,min_words_length:u8) {
        self.min_words_length = min_words_length;
    }

    pub fn pick_random_word(&self) -> &Word{
        // TODO shuffle
        let mut rng = rand::thread_rng();
        let random_number = rng.gen_range(0..=self.liste.len()-1); 
        self.liste.get(random_number).unwrap()
    }

    pub fn print(&self){
        for word in &self.liste   {
            println!("{}", word);
        }
    }
}

pub struct Word(pub String);

/* 
pub struct CountLetter{
    count: usize,
    positions: Vec<usize>
}

impl Word {
    pub fn count_letter(&self, l:char) -> CountLetter{
        let mut position = 0;
        let mut cl = CountLetter{count:0, positions:Vec::<usize>::new()};
        for c in self.0.chars() {
            if c == l {
                cl.count += 1;
                cl.positions.push(position);

            } 
            position += 1;
        }
        cl
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}
*/

impl Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}

impl Word {
    pub fn count_letter(&self, l:char) -> usize{
        let mut count = 0;
        let llc = l.to_lowercase().last().unwrap(); // On transforme la lettre en minuscule
        for c in self.0.chars() {
            if c.to_lowercase().last().unwrap() == llc {
                count += 1;

            } 
        }
        count
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}


pub struct HangmanGame<'a>{
    pub word_to_found:&'a Word,
    pub found_letters:Vec<char>,
    pub already_entered_chars:Vec<char>
}

impl<'a> HangmanGame<'a> {
    pub fn get_obfuscated_word(&self) -> String{
        let mut obsfu_word = String::new();
        for l in self.word_to_found.0.chars(){
            if self.found_letters.contains(&l) {
                obsfu_word.push(l);
            }else{
                obsfu_word.push('*');
            }
            obsfu_word.push(' ');
        }
        obsfu_word.to_uppercase().trim().to_owned()
    }

    pub fn get_word(&self) -> String{
        (*self.word_to_found.0).to_string().to_uppercase()
    }

    pub fn new(word:&'a Word) -> Self {
        HangmanGame{
            word_to_found: word,
            found_letters: Vec::<char>::new(),
            already_entered_chars: Vec::<char>::new(),
        }
    }

    pub fn save_entered_char(&mut self,c:char){
        self.already_entered_chars.push(c);
    }

    pub fn has_already_entered_char(&self,c:char) -> bool{
        self.already_entered_chars.contains(&c)
    }

    pub fn get_already_entered_chars(&self) -> String {
        let mut history = String::new();
        for l in &self.already_entered_chars {
            if history.chars().count() > 0 {
                history.push(',');
            }
            history.push(*l);
        }
        history.to_uppercase().trim().to_owned()
    }

    pub fn is_char_allowed(&self,letter:&str) -> bool {
        // Check if the input string has exactly one character
        if letter.len() == 1 {
            // Get the first and only character of the string
            let char_code = letter.chars().next().unwrap() as u32;
    
            // Check if the character code falls within the allowed range
            return (char_code >= 97 && char_code <= 122) || (char_code >= 65 && char_code <= 90)
        } 
        // Return false if the input string is empty or has more than one character
        false
    }

}