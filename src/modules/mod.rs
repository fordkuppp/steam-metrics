pub mod steam;

use std::time::Duration;
use async_trait::async_trait;
use tokio_cron_scheduler::Job;
use anyhow::Result;

#[async_trait]
pub trait TrackerModule: Send + Sync {
    fn name(&self) -> &'static str;
    fn init_meters(&self);
    async fn jobs(&self) -> Vec<Job>;
}

#[derive(Clone, Debug)]
pub struct ModuleConfig {
    pub poll_interval: Duration,
    pub enabled: bool,
    pub max_retries: u32,
}

impl Default for ModuleConfig {
    fn default() -> Self {
        Self {
            poll_interval: Duration::from_secs(60),
            enabled: true,
            max_retries: 3,
        }
    }
}

pub trait ModuleBuilder: Send + Sync {
    fn new() -> Self where Self: Sized;
    fn with_config(self, config: ModuleConfig) -> Self where Self: Sized;
    fn build(self) -> Result<Box<dyn TrackerModule>>;
}

pub struct ModuleRegistration {
    pub name: &'static str,
    pub builder_factory: fn() -> Box<dyn ModuleBuilder>,
}

inventory::collect!(ModuleRegistration);