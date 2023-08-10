use std::fs::{read_dir, metadata};
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
        if metadata(&asset_path).is_ok(){
         return GameObject{
                name,
                asset_path,
                game_object_type,
            };
        }
        println!("\n[WARN] While loading {name} exception happend, no file exists {asset_path}");
        println!("[WARN] Assiging {name} to a default asset");
        return GameObject{
            name,
            asset_path: "./src/asset/default.png".to_string(),
            game_object_type,
        };
    }
    pub fn save_local(&self){
        
    }
}


pub fn load_game_objects(path: String) -> Vec<GameObject>{
    let paths = read_dir(path).unwrap();
    let mut games : Vec<GameObject> = vec!();
    for path in paths{
    }
    games
}
