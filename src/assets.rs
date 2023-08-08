use std::fs;

enum AssetType{
    Image,
    Text,
    Audio,
}

pub struct Asset{
    pub path : String,
}

pub struct Map{
    image : Asset,
    name : String,
    size : (u8, u8),
}

pub struct Token{
    image : Asset,
    size : (u8, u8),
    position : (u8, u8),
}

pub fn load_assets() -> Vec<Asset>{
    let paths = fs::read_dir("./src/assets").unwrap();
    let mut assets : Vec<Asset> = vec!();
    for p in paths {
        assets.push(Asset {path : p.unwrap().path().to_str().unwrap().to_string()});
    }
    assets
}

