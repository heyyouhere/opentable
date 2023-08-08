use json::*;
use std::{fs::File, io::{Write, Read}};


pub enum GameObjectType {
    Token,
    Charsheet,
    Handout,
    Map,
}
pub struct GameObject{
    pub name : String,
    pub asset_path : String,
    game_object_type : GameObjectType,
}


impl GameObject {
    pub fn new(name : String, asset_path : String, game_object_type : GameObjectType) -> Self{
        if fs::metadata(&asset_path).is_ok(){
         return GameObject{
                name,
                asset_path,
                game_object_type,
            };
        }
        println!("\n[WARN] While loading {name} exception happend, no file exists {asset_path}");
        return GameObject{
            name,
            asset_path: "./src/asset/default.png".to_string(),
            game_object_type,
        };
    }
    pub fn save_local(&self, path : String){
        let j = object!{
            name : self.name.to_string(),
            asset_path : self.asset_path.to_string(),
        };
        let mut file = File::create(format!("{path}/{}.json", self.name.to_string())).unwrap();
        let _ = file.write_all(j.dump().as_bytes());
    }

   fn load_local(filepath: String) -> Self{
       let mut file = File::open(&filepath).unwrap();
       let mut contents = String::new();
       let _ = file.read_to_string(&mut contents);
       let parsed = json::parse(&contents).unwrap();
       let object = GameObject::new(
           parsed["name"].to_string(),
           parsed["asset_path"].to_string(),
           GameObjectType::Map,
        );
       object
   }
}

use std::fs;
pub fn load_game_objects(path: String) -> Vec<GameObject>{
    let paths = fs::read_dir(path).unwrap();
    let mut games : Vec<GameObject> = vec!();
    for p in paths {
        games.push(GameObject::load_local(p.unwrap().path().to_str().unwrap().to_string()));
    }
    games
}
