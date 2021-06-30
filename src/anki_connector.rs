use std::{collections::hash_map::DefaultHasher, hash::{Hash, Hasher}};

use reqwest::Client;
use serde::{Deserialize,Serialize};

#[derive(Debug,Serialize)]
struct AddNotesRequest {
    action: String,
    version: usize,
    params: NotesParams,
}

impl AddNotesRequest {

    pub fn new(params:NotesParams) -> Self {
        AddNotesRequest{ 
            action: String::from("addNotes"),
            version: 6,
            params: params
        }    
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn add_note(&mut self, note:AnkiNote){
        self.params.notes.push(note);
    }
}

impl Default for AddNotesRequest {

    fn default() -> Self {
        AddNotesRequest{ 
            action: String::from("addNotes"),
            version: 6,
            params: NotesParams::default()
        }    
    }
}

#[derive(Debug,Default, Serialize)]
struct NotesParams {
    notes:Vec<AnkiNote>,
}


#[derive(Debug,Default,Serialize)]
struct AnkiNote {
    deckName: &'static str,
    modelName: &'static str,
    fields: AnkiContent,
    tags: Vec<String>,
    picture:Vec<AnkiPicture>,
}

impl AnkiNote {

    pub fn new(deckName: &'static str, modelName: &'static str)-> Self {
        AnkiNote {
            deckName,
            modelName,
            fields: AnkiContent::default(),
            tags: Vec::new(),
            picture: Vec::new(),
        }
    }

    pub fn set_front(&mut self, name: &str){
        self.fields.Front = String::from(name);
    }

    pub fn set_back(&mut self, name: &str){
        self.fields.Back = String::from(name);
    }

    pub fn add_tag(&mut self, name: &str){
        self.tags.push(String::from(name));
    }
}

#[derive(Debug,Default, Serialize)]
struct AnkiPicture {
    url: String,
    filename: String,
    skipHash:String,
    fields: Vec<String>,
}

impl AnkiPicture {

    pub fn new(url: String, filename: String,fields: Vec<String>) -> AnkiPicture {
        let mut anki_p = AnkiPicture {
            url,
            filename,
            skipHash:String::new(),
            fields,
        };

        let mut hasher = DefaultHasher::new();
        let skip_hash = anki_p.hash(&mut hasher);
        s.finish();
        anki_p.skipHash = skip_hash;
        anki_p
    }
}


impl Hash for AnkiPicture {

    fn hash<H: Hasher>(&self, state: &mut H) {
        self.url.hash(state);
        self.filename.hash(state);
    }
}


#[derive(Debug,Default, Serialize)]
struct AnkiContent {
    Front:String,
    Back:String,
}

#[derive(Debug)]
struct AnkiClient {
    client:Client,
    url:String,
    method:String,
}

impl AnkiClient {

    fn new(){
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_anki_connect(){
        println!("{:#?}",AddNotesRequest::default());
    }
}