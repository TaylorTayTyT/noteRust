
use std::io::Write;
use std::{fmt::Write, fs}; 
use std::fs::File;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use rand::rngs::OsRng;
use rand_unique::{RandomSequence, RandomSequenceBuilder};

pub struct Note {
    pub title: String,
    pub description: String, 
    pub time_created: Option<Duration>, 
    pub id: u32
}

impl Note {
    fn new(title: String, description: String, time_created: Option<Duration>, id: u32) -> Note{
        Note{title, description, time_created, id}
    }
}

pub struct Notebook {
    name: String,
    num_notes: u32,
    sequence: RandomSequence<u32>
}

impl Notebook {
    pub fn new(name: String) -> Self{
        let result = fs::create_dir(name.clone());
        match result {
            Ok(_) => println!("Notebook successfully created!"),
            Err(err) => println!("Problem with making a directory. Is there a directory that already exists called {}?", name)
        }
        let config = RandomSequenceBuilder::<u32>::rand(&mut OsRng);
        let sequence = config.into_iter();
        Self {name, num_notes: 0, sequence}
    }
    pub fn get_name(&self) -> String{
        self.name.clone()
    }
    pub fn create_note(&mut self, note: &Note) -> Note{
        let id = self.sequence.next().unwrap();
        let mut file = File::create(note.title.clone());
        let duration = note.time_created.unwrap();
        let contents = String::from(duration.as_secs().to_string());
        file.unwrap().write_all(contents.as_bytes());
        Note::new(note.title.clone(), note.description.clone(), note.time_created, id)
    }
    pub fn delete_note(note: Note) -> Note{

    }
    pub fn update_note(note: Note) -> Note{

    }
    pub fn read_note(note: Note) -> Note{

    }
}

#[cfg(test)]

mod tests{
    use super::*;

    #[test]
    fn create_notebook(){
        let notebook = Notebook::new(String::from("taylor's Notebook"));
        assert_eq!(notebook.get_name(), "taylor's Notebook")
    }

    #[test]
    fn create_note(){
        let notebook = Notebook::new(String::from("tay's Notebook"));
    }

    #[test]
    fn read_note(){

    }
    #[test]
    fn update_note(){

    }
    #[test]
    fn delete_note(){

    }

}