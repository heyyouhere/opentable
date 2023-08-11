#![allow(dead_code)]



use std::time::SystemTime;
use std::fs::{File, read_dir};
use serde::*;
use std::io::{BufReader, BufWriter};



#[derive(Debug, Deserialize, Serialize)]
struct Player{
    nickname: String,
}

#[derive(Debug, Deserialize, Serialize)]
enum GameActionEnum{
    Roll,
    TextMessage,
    Connect,
    Disconnect,
}

#[derive(Debug, Deserialize, Serialize)]
struct GameAction{
    type_ : GameActionEnum,
    actor : Player,
    timestamp : SystemTime,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Game{
    name : String,
    id : u8,
    actions : Vec<GameAction>,
    maps : Vec<String>,
}
impl Game{
    pub fn new(name: String) -> Self{
        Game { 
            name,
            id : 0,
            actions : vec!(),
            maps : vec!(),
        }
    }
    
    pub fn save_local(&self){
        let file = File::create(format!("./src/games/{}.json", self.name)).unwrap();
        let _ = serde_json::to_writer(BufWriter::new(file), self);
    }
    fn load_local(filepath: String) -> Self{
        let file = File::open(filepath).unwrap();
        let game = serde_json::from_reader(BufReader::new(file)).unwrap();
        game
    }
}


pub fn load_games(path: &str) -> Vec<Game>{
    let paths = read_dir(path).unwrap();
    let games : Vec<Game> = paths.map(|p| Game::load_local(p.unwrap().path().to_str().unwrap().to_string())).collect();
    games
}




