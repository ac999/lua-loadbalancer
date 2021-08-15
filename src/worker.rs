use rlua::{Lua};
use tokio::sync::mpsc::{self, Sender};

use crate::types::{Worker, LuaCode, LuaResult};

impl Worker {
    pub fn new() -> Worker {
        let (tx, mut rx) = mpsc::channel(100);
        Worker {
            sender: tx,
            server: tokio::spawn(async move {
                let lua: Lua = Lua::new();
                loop {
                    match rx.recv().await {
                        Some((lua_code, res)) => {
                            lua.context(|lua_context| {
                                lua_context.load(&lua_code.code)
                                    .exec()
                            }).unwrap();
                            res.clone()
                                .send(LuaResult { int_result: Some(0), string_result: Some("0".to_string()) })
                                .await;
                        },
                        _ => {},
                    }
                }
            }),
        }
    }

    pub async fn do_work(&self, res: (LuaCode, Sender<LuaResult>)) {
        self.sender.clone().send(res).await;
    }
}
