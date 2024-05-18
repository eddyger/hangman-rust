use select::predicate::Name;
use select::predicate::Class;
use select::document::Document;


pub struct Definition {

}

impl Definition {
    pub fn get_word_definition(&self, word_id:String) -> Result<String, Box<dyn std::error::Error>>{
        let language_code = "fr";

        let url = format!("https://{}.wiktionary.org/wiki/{}", language_code, word_id.to_lowercase());

        let body = reqwest::blocking::get(&url)?.text()?;

        let document = Document::from(body.as_str());

        // Find the element containing the definition
        let div_content_element = document.find(Class("mw-parser-output")).next().unwrap();

        let mut response = div_content_element.find(Name("p")).next().unwrap().text();
        
        if div_content_element.find(Name("dd")).next().is_some(){
            response.push_str(&div_content_element.find(Name("dd")).next().unwrap().text());
        }
        
        let definitions = div_content_element.find(Name("ol")).next();
        if definitions.is_some(){
            let definitions = definitions.unwrap().find(Name("li"));
            for definition in definitions {
                response.push_str(&definition.text());
            }
        }
        
        Ok(response)

    }
}