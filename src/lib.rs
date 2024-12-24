
use std::fs; 
use std::io; 
use rand::rngs::OsRng;
use rand_unique::{RandomSequence, RandomSequenceBuilder};

pub struct Note {
    pub title: String,
    pub description: String, 
    pub time_created: u64, 
    pub id: u32
}

impl Note {
    fn new(title: String, description: String, time_created: u64, id: u32) -> Note{
        Note{title, description, time_created, id}
    }
}

pub struct Notebook {
    name: String,
    num_notes: u32,
    config: RandomSequenceBuilder<u32>
}

impl Notebook {
    pub fn new(name: String) -> Self{
        let result = fs::create_dir(name.clone());
        match result {
            Ok(_) => println!("Notebook successfully created!"),
            Err(err) => println!("Problem with making a directory. Is there a directory that already exists called {}?", name)
        }
        let config = RandomSequenceBuilder::<u32>::rand(&mut OsRng);
        Self {name, num_notes: 0, config}
    }
    pub fn get_name(&self) -> String{
        self.name.clone()
    }
    pub fn create_note(note: Note) -> Note{
        
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