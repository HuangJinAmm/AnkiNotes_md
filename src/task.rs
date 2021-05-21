use chrono::{DateTime,Utc};
use serde::Deserialize;
use serde::Serialize;

use std::io::Result;
use std::Path::PathBuf;

#[derive(Debug,Deserialize,Serialize)]
pub struct Task {
    pub text: String,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(text:String) -> Task {
        let created_at:DateTime<Utc> = Utc::now();
        Task {text,created_at}
    }
}

pub fn add_task(jouranl_path:PathBuf, task: Task) -> Result<()> {
    todo!()
}

pub fn complete_task(jouranl_path:PathBuf,task_position:usize) -> Result<()> {
    todo!()
}

pub fn list_tasks(jouranl_path:PathBuf) -> Result<()> {
    todo!()
}