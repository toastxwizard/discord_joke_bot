use std::path::Path;
use std::process::{Command, Stdio};
use std::io::Write;
use std::thread;
use std::time;
use std::sync::Arc;

pub struct Joke {
    conversion_file: String,
    final_file: String
}

impl Joke {
    pub fn new(joke_string: String, name: &String) -> Self {
        let final_file_name : String = format!("jokes/{}_final.wav", name.clone());
        let conversion_file_name : String = format!("jokes/{}.wav", name.clone());
        
        let joke = Joke{
            final_file: final_file_name,
            conversion_file: conversion_file_name,
        };

        //create conversion file
        joke.create_conversion_file(&joke_string);
                
        //create final file
        joke.create_final_file();

        return joke;
    }

    pub fn init(){
        if Path::new("jokes").exists() {
            std::fs::remove_dir_all("jokes").expect("Could not delete jokes folder");
        }

        std::fs::create_dir("jokes").expect("Could not create jokes folder");
    }

    pub fn get_joke_file_path(&self) -> String{
        self.final_file.clone()
    }

    pub fn clean_up_files(&self){
        std::fs::remove_file(self.final_file.clone()).expect("Could not delete file");
        std::fs::remove_file(self.conversion_file.clone()).expect("Could not delete file");
    }

    /* Private methods */

    fn create_conversion_file(&self, joke: &String){
        let espeak_process = Command::new("espeak")
            .arg("--stdin")
            .arg("-m")
            .arg("-w")
            .arg(self.conversion_file.clone())
            .stdin(Stdio::piped())
            .spawn()
            .expect("Error creating buffer file");

        espeak_process.stdin.unwrap()
            .write_all(joke.as_bytes())
            .expect("Error generating tts file");
    }

    fn create_final_file(&self){
        Command::new("ffmpeg")
            .arg("-i")
            .arg(self.conversion_file.clone())
            .arg(self.final_file.clone())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .expect("Error converting to ffmpeg format");
    }
}