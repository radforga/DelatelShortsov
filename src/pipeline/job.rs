use std::path::PathBuf;
use std::time::Duration;

use crate::pipeline::job::VideoSource::Url;

pub struct Job {
 pub source: VideoSource,
 pub clips: Vec<Clip>,
 pub template: String,
 
}

impl Job{
    pub fn new() -> Self {
        Self { source: (VideoSource::Url(String::from("8"))), clips: (Vec::new()), template: (String::from("1")) }
    }
    pub fn edit_source_url(&mut self,url : String){
        self.source = VideoSource::Url(url);   
    }

}

pub enum VideoSource{
    Url(String),
    File(PathBuf),
}

pub struct Clip {
 pub start: Duration,
 pub end: Duration,
 pub name: Option<String>,
}
