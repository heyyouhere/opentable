#![allow(dead_code)]

mod gameobjects;

use json::*;
use std::fs::File;
use std::io::prelude::*;



#[derive(Debug)]
struct Player{
    nickname: String,
}

#[derive(Debug)]
enum GameActionEnum{
    Roll,
    TextMessage,
    Connect,
    Disconnect,
}

struct GameAction{
    type_ : GameActionEnum,
    actor : Player,
    timestamp : SystemTime,
}

struct Game{
    name : String,
    id : u8,
    actions : Vec<GameAction>,
    
}
impl Game{
    fn new(name: String) -> Self{
        Game { 
            name,
            id : 0,
            actions : vec!(),
        }
    }
    
    fn save_local(&self, path : String){
        let j = object!{
            name : self.name.to_string(),
            id : self.id,
        };
        let mut file = File::create(format!("{path}/{}.json", self.name.to_string())).unwrap();
        //TODO: Raise exception on file write error
        let _ = file.write_all(j.dump().as_bytes());
    }

    fn load_local(filepath: String) -> Self{
        let mut file = File::open(&filepath).unwrap();
        let mut contents = String::new();
        let _ = file.read_to_string(&mut contents);
        let parsed = json::parse(&contents).unwrap();
        let game = Game {
            name: parsed["name"].to_string(),
            id: parsed["id"].as_u8().unwrap(),
            actions : vec!(),
        };
        game
    }
}

use std::time::SystemTime;
use std::fs;

fn load_games(path: String) -> Vec<Game>{
    let paths = fs::read_dir(path).unwrap();
    let mut games : Vec<Game> = vec!();
    for p in paths {
        games.push(Game::load_local(p.unwrap().path().to_str().unwrap().to_string()));
    }
    games
}

fn main() {
    let games =  load_games("./src/games".to_string());
    println!("Loaded {} games:", games.len());
    for g in games{
        println!("  {}", g.name);
    }

    let game_objects = gameobjects::load_game_objects("./src/gameobjects".to_string());
    println!("\nLoaded {} game_objects", game_objects.len());
    for go in game_objects{
        println!("  {} {}", go.name, go.asset_path);
    }
  

    //let new_game = Game::new("SavageWorldGame".to_string());
    //new_game.save_local("./src/games".to_string());
}
