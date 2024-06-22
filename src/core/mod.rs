use crate::Task;

pub mod manager;
mod os;

pub trait Scheduler {
    async fn start(&mut self, task: Task) -> anyhow::Result<()>;
    async fn monitor(&mut self);
    async fn restart(&mut self, task: Task) -> anyhow::Result<()>;
    async fn stop(&self, task: Task);
}