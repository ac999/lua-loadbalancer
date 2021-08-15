use crate::types::{LoadBalancer, Worker, LuaCode, LuaResult};

use rand::{thread_rng, Rng};
use tokio::sync::mpsc::Sender;

impl LoadBalancer {
    pub fn new(num: u32) -> Result<LoadBalancer, &'static str> {
        let mut pool: Vec<Worker> = Vec::new();
        for i in 0..num {
            pool.push(Worker::new());
        }
        Ok(LoadBalancer { pool })
    }

    pub async fn assign_task(&mut self, res: (LuaCode, Sender<LuaResult>)) {
        let mut rng = thread_rng();
        let worker_nr: usize = rng.gen_range(0..self.pool.len());
        let mut worker = &self.pool[worker_nr];
        worker.do_work(res).await;
    }
}
