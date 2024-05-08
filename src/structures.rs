pub mod structures {
    use std::{fmt::Display, fs, io::{BufRead, BufReader, Error}};

    use rand::Rng;


    pub struct Dictionnary {
        pub liste:Vec<Word>,
    }

    impl Dictionnary {
        pub fn load_from_file_path(file_path:String) -> Result<Self, Error> {
            // Open the file and handle the result with the ? operator
            let file = fs::File::open(file_path)?;

            // Create a new instance of Dictionnary with an empty vector
            let mut dictionnary = Dictionnary {
                liste: Default::default(),
            };

            // fill the dictionnary
            let reader = BufReader::new(file);

            for line in reader.lines() {
                let line = line?;
                dictionnary.liste.push(Word(line));
            }

            Ok(dictionnary)
            
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

    pub struct Word(String);

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
        pub found_letters:Vec<char>
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
            obsfu_word.to_uppercase()
        }

        pub fn get_word(&self) -> String{
            (*self.word_to_found.0).to_string().to_uppercase()
        }
    }


}