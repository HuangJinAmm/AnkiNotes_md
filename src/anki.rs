use genanki_rs::{Deck, Field, Note, Template};
use genanki_rs::{Model, Package};
use pulldown_cmark::{html, Options, Parser};
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io::Result;
use std::ops::Add;
use std::path::PathBuf;
use std::str;
use walkdir::WalkDir;

const note_key: &str = "note_seperator";
const question_key: &str = "question_seperator";

struct MyDeck {
    my_model: Model,
    deck: Deck,
    imgs: Vec<String>,
}

impl MyDeck {
    pub fn new(id: usize, name: &'static str, decription: &'static str) -> Self {
        let custom_css = ".card {\n font-family: arial;\n font-size: 20px;\n color: black;\n}\n";
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
            imgs: Vec::new(),
        }
    }

    pub fn find_and_add_image(&mut self, text: &str) -> String {
        let re = Regex::new("!\\[.+?\\]\\((.+?)\\)").unwrap();
        let mut res = String::from(text);

        for result in re.captures_iter(text) {
            let path = result.get(1).unwrap().as_str();
            self.imgs.push(String::from(path));
            let p = path.split('\\').last().unwrap();
            res = res.replace(path, p);
        }
        res
    }

    pub fn add_note(&mut self, card: MyCard) {
        let q1 = self.find_and_add_image(card.0.as_str());
        let q2 = self.find_and_add_image(card.1.as_str());

        let qusetion = parse_to_html(q1.as_str());
        let answer = parse_to_html(q2.as_str());

        self.deck.add_note(
            Note::new(
                self.my_model.clone(),
                vec![qusetion.as_str(), answer.as_str()],
            )
            .unwrap(),
        );
    }

    pub fn write(self, file: String) {
        let imgs = self.imgs.iter().map(|im| im.as_str()).collect();
        let mut p = Package::new(vec![self.deck], imgs).unwrap();
        p.write_to_file(file.as_str()).unwrap();
    }
}

struct MarkDownParser {
    note_sep: String,
    qusetion_sep: String,
    notes: Vec<MyCard>,
}

struct MyCard(String, String);

impl Default for MarkDownParser {

    fn default() -> Self {
        MarkDownParser{
            note_sep:String::from("\r\n\r\n\r\n"),
            qusetion_sep:String::from("---"),
            notes: Vec::new(),
        }
    }
}

impl MarkDownParser {
    pub fn new(note_sep: String, qusetion_sep: String) -> Self {
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
        // let temp = contents.replace("\r\n", "\n");

        let note_seperator = self.note_sep.clone();

        let cards = contents.split(note_seperator.as_str());

        for card in cards {
            if let Some((q, a)) = card.split_once(self.qusetion_sep.clone().as_str()) {
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
    let mut setting = config::Config::default();
    let mut mark_down_parser = MarkDownParser::default();
    if let Ok(_) = setting.merge(config::File::with_name("setting")){
        let config = setting.try_into::<HashMap<String, String>>().unwrap();
        let default_note_seq = &String::from("\r\n\r\n\r\n");
        let default_qust_seq = &String::from("---");
        let note_sep = config.get(note_key).unwrap_or(&default_note_seq);
        let qusetion_sep = config.get(question_key).unwrap_or(&default_qust_seq);
        mark_down_parser = MarkDownParser::new(note_sep.to_owned(), qusetion_sep.to_owned());
    }

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

pub fn generate_apkg_from_current_dir() -> Result<()>{
    for entry in WalkDir::new(".").max_depth(1).into_iter() {
        let entry = entry.unwrap();
        if let Some(file_name) = entry.file_name().to_str() {
            if file_name.ends_with(".md") {
                let file_name = file_name.get(..(file_name.len() - 3)).unwrap();
                generate_apkg(String::from(file_name), String::from(file_name))?;
            }
        }
    }
    Ok(())
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
    fn test_parse_img() {
        let mut my_deck = MyDeck::new(123454, "test_img", "test_img_decription");
        let r = my_deck.find_and_add_image(
            r"axs![aaaa](E:\code\rust\AnkiNotes_md\image\test\2021-06-21.png)jhgn",
        );
        println!("{}", r);
    }
}
