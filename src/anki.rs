use genanki_rs::Model;
use genanki_rs::{Deck, Field, Note, Template};
use pulldown_cmark::{html, Options, Parser};
use std::fs;
use std::io::Result;
use std::ops::Add;
use std::path::PathBuf;
use std::str;
use walkdir::WalkDir;

struct MyDeck {
    my_model: Model,
    deck: Deck,
}

impl MyDeck {
    pub fn new(id: usize, name: &'static str, decription: &'static str) -> Self {
        let custom_css = ".card {\n font-family: arial;\n font-size: 20px;\n text-align: center;\n color: black;\n}\n";
        let my_model = Model::new_with_options(
            1607392319,
            "Simple Model",
            vec![Field::new("Question"), Field::new("Answer")],
            vec![Template::new("Card")
                .qfmt("{{Question}}")
                .afmt(r#"{{FrontSide}}<hr id="answer">{{Answer}}"#)],
            Some(custom_css),
            None,
            None,
            None,
            None,
        );
        let deck = Deck::new(id, name, decription);
        MyDeck {
            my_model,
            deck,
        }
    }

    pub fn add_note(&mut self, card: MyCard) {
        let qusetion = card.0.as_str();
        let answer = card.1.as_str();

        self.deck.add_note(
            Note::new(
                self.my_model.clone(),
                vec![
                    parse_to_html(qusetion).as_str(),
                    parse_to_html(answer).as_str(),
                ],
            )
            .unwrap(),
        );
    }

    pub fn write(self, file: String) {
        self.deck.write_to_file(file.as_str()).unwrap();
    }
}

struct MarkDownParser {
    note_sep: &'static str,
    qusetion_sep: &'static str,
    notes: Vec<MyCard>,
}

struct MyCard(String, String);

impl MarkDownParser {
    pub fn new(note_sep: &'static str, qusetion_sep: &'static str) -> Self {
        MarkDownParser {
            note_sep,
            qusetion_sep,
            notes: Vec::new(),
        }
    }

    pub fn add_card(&mut self, card: MyCard) {
        self.notes.push(card);
    }

    pub fn parse_from_file(&mut self, file: PathBuf) {
        let contents = fs::read_to_string(file).expect("无法打开文件");
        let temp = contents.replace("\r\n", "\n");

        let cards = temp.split(self.note_sep);

        for card in cards {
            if let Some((q,a)) = card.split_once(self.qusetion_sep){
                self.add_card(MyCard(String::from(q), String::from(a)));
            }
        }
    }
}

pub fn parse_to_html(input: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);

    let parser = Parser::new_ext(input, options);

    let mut html_output = String::new();

    html::push_html(&mut html_output, parser);

    html_output
}

pub fn generate_apkg(md: String, apkg: String) -> Result<()> {
    let mut mark_down_parser = MarkDownParser::new("\n\n\n", "---");
    let md_path = PathBuf::from(md.add(".md"));
    let apkg_path = apkg.add(".apkg");
    mark_down_parser.parse_from_file(md_path);
    let mut my_deck = MyDeck::new(123453, "test_rust", "test_from_rust");
    for card in mark_down_parser.notes.into_iter() {
        my_deck.add_note(card);
    }
    my_deck.write(apkg_path);
    Ok(())
}

pub fn generate_apkg_from_current_dir() {
    for entry in WalkDir::new(".").max_depth(1).into_iter() {
        let entry = entry.unwrap();
        if let Some(file_name) = entry.file_name().to_str() {
            if file_name.ends_with(".md") {
                let file_name = file_name.get(..(file_name.len() -3)).unwrap();
                generate_apkg(String::from(file_name), String::from(file_name)).unwrap();
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_create_anki() {
        let mut my_deck = MyDeck::new(1234, "test_rust", "test auto create anki from rust");
        my_deck.add_note(MyCard(
            String::from("<h2>qusetion1</h2>"),
            String::from("<h3>answer1</h3>"),
        ));
        my_deck.add_note(MyCard(
            String::from("<h3>qusetion2</h3>"),
            String::from("<h3>answer2</h3>"),
        ));
        my_deck.write(String::from("test.apkg"));
    }

    #[test]
    fn test_parse_markdown() {
        let markdown_input = "### hello world \r\n *something*";

        let html_output = parse_to_html(markdown_input);

        println!("{}", html_output);
    }

    #[test]
    fn test_walk_dir() {}
}
