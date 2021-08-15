use serde::{Deserialize, Serialize};

use tokio::sync::mpsc::Sender;
use tokio::task::JoinHandle;

#[derive(Serialize, Deserialize, Debug)]
pub struct LuaCode {
    pub code: String,
}

pub struct LuaResult {
    pub int_result: Option<i64>,
    pub string_result: Option<String>,
}

pub struct Worker {
    pub sender: Sender<(LuaCode, Sender<LuaResult>)>,
    pub server: JoinHandle<u32>,
}

pub struct LoadBalancer {
    pub pool: Vec<Worker>,
}