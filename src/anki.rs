use std::fs;
use std::io::Result;
use std::path::PathBuf;
use genanki_rs::Model;
use genanki_rs::{Template, Deck, Note,Field};

struct MyDeck {
    id: usize,
    name: &'static str,
    decription: &'static str,
    my_model:Model,
    deck:Deck
}

impl MyDeck {

    pub fn new(id: usize,
         name: &'static str,
         decription:&'static str,
         my_model:Model) ->Self {
        let deck = Deck::new(id, name, decription);
        MyDeck {
            id,name,decription,my_model,deck
        }
    }

    pub fn add_note(&mut self,qusetion: String,answer: String){
        self.deck.add_note(Note::new(self.my_model.clone(),vec![qusetion.as_str(),answer.as_str()]).unwrap());
    }

    pub fn write(self,file:&'static str){
        self.deck.write_to_file(file);
    }
}

struct MarkDownParser {
    note_sep: &'static str,
    qusetion_sep:&'static str,
    notes: Vec<MyCard>,
}

struct MyCard(String,String);

impl MarkDownParser {

    pub fn new(note_sep:&'static str,qusetion_sep:&'static str) -> Self {
        MarkDownParser {
            note_sep,
            qusetion_sep,
            notes:Vec::new(),
        }

    }

    pub fn add_card(&mut self,card:MyCard) {
        self.notes.push(card);
    }

    pub fn parse_from_file(&mut self,file:&'static str) {
        let contents = fs::read_to_string(file).expect("无法打开文件");
        let temp = contents.replace("\r\n","\n");
        let cards = temp.split(self.note_sep);

        for card in cards{
            let split:Vec<&str> = card.split(self.qusetion_sep).collect();
            self.add_card(MyCard(String::from(split[0]), String::from(split[1])));
        }

    }
}

pub fn generate_apkg(filename: PathBuf) ->Result<()>{
    let my_model = Model::new(
        1607392319,
        "Simple Model",
        vec![Field::new("Question"), Field::new("Answer")],
        vec![Template::new("Card 1")
            .qfmt("{{Question}}")
            .afmt(r#"{{FrontSide}}<hr id="answer">{{Answer}}"#)],
    );
    Ok(())
}

