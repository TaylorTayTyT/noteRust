
use std::io::Write;
use std::path::PathBuf;
use std::{env, fs}; 
use std::fs::{File, OpenOptions};
use std::time::{Duration};
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
    sequence: RandomSequence<u32>,
    parent_dir: PathBuf
}

impl Notebook {
    pub fn new(name: String) -> Result<Self, String>{
        let result = fs::create_dir(name.clone());
        match result {
            Ok(_) => println!("Notebook successfully created!"),
            Err(err) => {
                println!("Problem with making a directory. Is there a directory that already exists called {}?", name);
                return Err(String::from(err.to_string()))
            }
        }
        let config = RandomSequenceBuilder::<u32>::rand(&mut OsRng);
        let sequence = config.into_iter();
        let parent_dir = env::current_dir().unwrap(); 
        Ok(Self {name, num_notes: 0, sequence, parent_dir})
    }
    pub fn get_name(&self) -> String{
        self.name.clone()
    }
    pub fn create_note<'a>(&mut self, note: &'a Note) -> Result<&'a Note, String>{
        let dir = env::current_dir().unwrap();
        let file_path = dir.join(self.get_name()).join(&note.title);
        let duration = note.time_created.unwrap();
        let contents = String::from(format!("{}\n{}", duration.as_secs().to_string(), &note.description));
        let mut file = OpenOptions::new().write(true).create(true).open(file_path).unwrap();
        let _ = file.write_all(&contents.as_bytes());
        Ok(note)
    }
    pub fn delete_note(&self, note_title: &String) -> String{
        let dir_path = self.parent_dir.clone();
        let note_path = dir_path.join(note_title);
        let _ = match fs::exists(&note_path).unwrap(){
            true => fs::remove_file(&note_path),
            false => return String::from("File not found"),
            _ => return String::from("File not found")
        };
        String::from("deleted")
    }
    pub fn update_note(note: Note) -> Note{
        Note { title: String::from("sample"), description: String::from("sample"), time_created: Some(Duration::new(100, 100)), id: 4 }
    }
    pub fn read_note(note: Note) -> Note{
        Note { title: String::from("sample"), description: String::from("sample"), time_created: Some(Duration::new(100, 100)), id: 4 }
    }
    pub fn delete(&mut self) {
        let mut directory = String::from("C:\\Users\\taylo\\Downloads\\testing\\");
        directory.push_str(self.name.clone().as_str());
        println!("{}", directory);
        fs::remove_dir_all(directory.as_str()).expect("Directory should have been removed");
        let _ = self;
    }
}

#[cfg(test)]

mod tests{

    use std::env;

    use super::*;

    #[test]
    fn create_notebook(){
        let notebook = Notebook::new(String::from("taylor's Notebook"));
        assert_eq!(notebook.as_ref().unwrap().get_name(), "taylor's Notebook");
        let directory_path = env::current_dir().unwrap();
        let notebook_path = directory_path.join("taylor's Notebook");
        assert!(fs::exists(notebook_path).unwrap());
        notebook.unwrap().delete();
    }

    #[test]

    fn delete_notebook(){
        let notebook = Notebook::new(String::from("to be deleted Notebook"));
        let directory_path = env::current_dir().unwrap();
        let notebook_path = directory_path.join("to be deleted Notebook");
        notebook.unwrap().delete();
        assert!(!fs::exists(notebook_path).unwrap());
    }

    #[test]
    fn create_note(){
        let notebook = Notebook::new(String::from("tay's Notebook"));
        let note = Note::new(String::from("title"), String::from("description"), Some(Duration::new(10, 10)), 4 as u32);
        let mut unwrapped_notebook = notebook.unwrap(); 
        let _ = &unwrapped_notebook.create_note(&note);
        let directory_path = env::current_dir().unwrap();
        let notebook_path = directory_path.join("tay's Notebook").join(&note.title);
        assert!(fs::exists(notebook_path).unwrap());
    }

    #[test]
    fn read_note(){
        return; 
    }
    #[test]
    fn update_note(){
        return; 
    }
    #[test]
    fn delete_note(){
        return; 
    }

}