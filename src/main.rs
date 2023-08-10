#![allow(dead_code)]

mod gameobjects;
mod maps;


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
struct Game{
    name : String,
    id : u8,
    actions : Vec<GameAction>,
    maps : Vec<String>,
}
impl Game{
    fn new(name: String) -> Self{
        Game { 
            name,
            id : 0,
            actions : vec!(),
            maps : vec!(),
        }
    }
    
    fn save_local(&self, path : String){
        let file = File::create(format!("{path}/{}.json", self.name)).unwrap();
        let _ = serde_json::to_writer(BufWriter::new(file), self);
    }
    fn load_local(filepath: String) -> Self{
        let file = File::open(filepath).unwrap();
        let game = serde_json::from_reader(BufReader::new(file)).unwrap();
        game
    }
    fn add_map(&mut self, map : maps::Map){
        self.maps.push(map.path);
    }
}


fn load_games(path: String) -> Vec<Game>{
    let paths = read_dir(path).unwrap();
    let games : Vec<Game> = paths.map(|p| Game::load_local(p.unwrap().path().to_str().unwrap().to_string())).collect();
    games
}
mod server;
fn main(){
    server::serve("0.0.0.0:1583");

}
fn main2() {
    let action = GameAction{
            type_ : GameActionEnum::TextMessage,
            actor : Player{nickname:  "heyyouhere".to_string()},
            timestamp : SystemTime::now(),
        };
    /*
        let my_game = Game{
        name: "Serde-Game".to_string(),
        id : 0,
        actions : vec!(),
        maps: vec!(),
    };
    my_game.save_local("src/".to_string());
    */

    let mut loaded_game = Game::load_local("src/Serde-Game.json".to_string());
    loaded_game.actions.push(action);
    loaded_game.save_local("src/".to_string());

}


/*        --> Map  
 *   Game
 *
 *
 *
 *
 *
 */







