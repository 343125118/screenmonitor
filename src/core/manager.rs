use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex};
use tokio::select;
use crate::Task;
use tokio::sync::broadcast;
use tracing::{error, info};
use crate::core::Scheduler;
use subprocess::{Exec, Popen};
use anyhow::Result;

pub struct Manager {
    pub name: String,
    pub tasks: Arc<Mutex<HashMap<String, Task>>>,
    pub children: Arc<Mutex<HashMap<String, Arc<Mutex<Popen>>>>>,
    pub closer: broadcast::Receiver<String>,
}

impl Manager {
    pub fn new(closer: broadcast::Receiver<String>) -> Self {
        let name = "WindowsCore".to_string();
        let tasks = Arc::new(Mutex::new(HashMap::new()));
        let children = Arc::new(Mutex::new(HashMap::new()));
        Manager {
            name,
            tasks,
            closer,
            children,
        }
    }
}

impl Scheduler for Manager {
    async fn start(&mut self, task: Task) -> Result<()> {
        self.tasks.lock().await.insert(task.id.clone(), task.clone());

        #[cfg(target_os = "macos")]
            let exec = Exec::cmd(&task.command)
            .arg("-f")
            .arg("avfoundation")
            .arg("-i")
            .arg("1")
            .arg("-r")
            .arg("30")
            .arg("-s")
            .arg("1920x1080")
            .arg("-vcodec")
            .arg("libx264")
            .arg("-preset")
            .arg("ultrafast")
            .arg("-crf")
            .arg("18")
            .arg("-pix_fmt")
            .arg("yuv420p")
            .arg("output.mp4")
            .arg("-y")
            .popen()?;

        #[cfg(target_os = "windows")]
            let exec = Exec::cmd(&task.command)
            .arg("-f")
            .arg("gdigrab")
            .arg("-framerate")
            .arg("30")
            .arg("-i")
            .arg("desktop")
            .arg("-vcodec")
            .arg("libx264")
            .arg("-preset")
            .arg("ultrafast")
            .arg("-crf")
            .arg("18")
            .arg("-pix_fmt")
            .arg("yuv420p")
            .arg("output.mp4")
            .arg("-y")
            .popen()?;

        let child = Arc::new(Mutex::new(exec));
        self.children.lock().await.insert(task.id.clone(), Arc::clone(&child));
        let children = Arc::clone(&self.children);
        println!("Started task {} with id {}", task.name, task.id);
        tokio::spawn(async move {
            let mut c = child.lock().await;
            match c.wait() {
                Ok(exit_code) => {
                    info!("Task {} with id {} has stopped", task.name, task.id);
                    children.lock().await.remove(&task.id).unwrap();
                }
                Err(_) => {
                    error!("Failed to start task {}", task.name);
                }
            }
        });
        Ok(())
    }


    async fn monitor(&mut self) {
        let mut closer = self.closer.resubscribe();
        loop {
            let timer = tokio::time::sleep(tokio::time::Duration::from_secs(1));
            let mut tasks_to_restart = Vec::new();
            {
                let tasks = self.tasks.lock().await;
                let children = self.children.lock().await;
                for (k, v) in tasks.iter() {
                    if !children.contains_key(k) {
                        tasks_to_restart.push(v.clone());
                    }
                }
            }
            for task in tasks_to_restart {
                let _ = self.start(task).await;
            }
            select! {
            _ = closer.recv() => {
                break;
            }
            _ = timer => {}
        }
        }
    }

    async fn restart(&mut self, task: Task) -> Result<()> {
        self.stop(task.clone()).await;
        self.start(task).await
    }

    async fn stop(&self, task: Task) {
        if let Some(clild) = self.children.lock().await.get(&task.id) {
            let mut child = clild.lock().await;
            match child.kill() {
                Ok(_) => {
                    error!("Stopped task {} with id {}", task.name, task.id);
                }
                Err(e) => {
                    error!("Failed to stop task {}: {}", task.name, e);
                }
            }
        }
    }
}