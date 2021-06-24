use reqwest::Client;

#[derive(Debug)]
struct AddNotesRequest {
    action: String,
    version: usize,
    params: NotesParams,
}

impl AddNotesRequest {

    pub fn new() {
        
    }
}

#[derive(Debug)]
struct NotesParams {
    notes:AnkiNote,
}

impl NotesParams {

    fn new() -> NotesParams {

    }
}

#[derive(Debug)]
struct AnkiNote {
    deckName: String,
    modelName: String,
    fields: AnkiContent,
    tags: Vec<String>,
    picture:Vec<AnkiPicture>,
}

impl AnkiNote {

    fn new() -> AnkiNote {
        
    }
}

#[derive(Debug)]
struct AnkiPicture {
    url: String,
    filename: String,
    skipHash:String,
    fields: Vec<String>,
}

impl AnkiPicture {

    fn new() -> AnkiPicture {
        
    }
}

#[derive(Debug)]
struct AnkiContent {
    Front:String,
    Back:String,
}

impl AnkiContent {

    fn new() -> AnkiContent {

    }
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
