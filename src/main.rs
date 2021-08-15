mod worker;
mod types;
mod load_balancer;

use std::fs::File;
use std::io::Read;

use rlua::{Lua};
use serde_json;

use types::{LuaCode, LoadBalancer};
use tokio::sync::mpsc::Sender;
use tokio::sync::mpsc;


fn read_file(path: String) -> String {
    let mut data: String = String::new();
    File::open(path).unwrap().read_to_string(&mut data).unwrap();
    data
}

fn load_lua_script(path: String) -> Result<Vec<LuaCode>, serde_json::Error> {
    let data: String = read_file(path);
    let lua: Vec<LuaCode> = serde_json::from_str(&data)?;
    Ok(lua)
}
#[tokio::main]
async fn main() {
    let lua_list: Vec<LuaCode> = load_lua_script("./lua.json".to_string()).unwrap();
    let lua: Lua = Lua::new();
    let mut lb: LoadBalancer = LoadBalancer::new(5).unwrap();
    for script in lua_list {
        let (tx, mut rx) = mpsc::channel(100);
        lb.assign_task((script, tx)).await;
    }
}
