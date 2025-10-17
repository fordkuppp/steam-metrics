mod meter;

use std::env;
use std::sync::OnceLock;
use std::time::Duration;
use crate::modules::{ModuleBuilder, ModuleConfig, ModuleRegistration, TrackerModule};
use async_trait::async_trait;
use tokio_cron_scheduler::Job;
use opentelemetry::{global, metrics::Meter};
use lazy_static::lazy_static;
use opentelemetry::metrics::{Counter, Histogram};
use anyhow::{Context, Result};

pub static STEAM_SUMMARY_LATENCY: OnceLock<Histogram<f64>> = OnceLock::new();
pub static STEAM_SUMMARY_ERRORS_TOTAL: OnceLock<Counter<u64>> = OnceLock::new();
pub static STEAM_GAME_TIME_TOTAL: OnceLock<Counter<u64>> = OnceLock::new();

pub struct SteamModule {
    api_key: String,
    steam_ids: Vec<String>,
    poll_interval: Duration,
    max_retries: u32,
    enabled: bool,
}

pub struct SteamModuleBuilder {
    api_key: Option<String>,
    steam_ids: Option<Vec<String>>,
    config: ModuleConfig,
}

impl ModuleBuilder for SteamModuleBuilder {
    fn new() -> Self {
        Self {
            api_key: None,
            steam_ids: None,
            config: ModuleConfig::default(),
        }
    }

    fn with_config(mut self, config: ModuleConfig) -> Self {
        self.config = config;
        self
    }

    fn build(self) -> Result<Box<dyn TrackerModule>> {
        let api_key = self.api_key
            .or_else(|| env::var("STEAM_API_KEY").ok())
            .context("STEAM_API_KEY not configured")?;

        let steam_ids = self.steam_ids
            .or_else(|| {
                env::var("STEAM_IDS").ok().map(|s| {
                    s.split(',').map(|id| id.trim().to_string()).collect()
                })
            })
            .context("STEAM_IDS not configured")?;

        Ok(Box::new(SteamModule {
            api_key,
            steam_ids,
            poll_interval: self.config.poll_interval,
            max_retries: self.config.max_retries,
            enabled: self.config.enabled,
        }))
    }
}

impl SteamModuleBuilder {
    pub fn with_api_key(mut self, api_key: String) -> Self {
        self.api_key = Some(api_key);
        self
    }

    pub fn with_steam_ids(mut self, steam_ids: Vec<String>) -> Self {
        self.steam_ids = Some(steam_ids);
        self
    }

    pub fn with_poll_interval(mut self, interval: Duration) -> Self {
        self.config.poll_interval = interval;
        self
    }
}

#[async_trait]
impl TrackerModule for SteamModule {
    fn name(&self) -> &'static str {
        "steam"
    }

    fn init_meters(&self) {
        if !self.enabled {
            println!("Steam module disabled, skipping meter initialization");
            return;
        }

        let meter = global::meter(self.name());

        STEAM_SUMMARY_LATENCY.get_or_init(|| {
            meter.f64_histogram("steam_summary_latency")
                .with_description("The duration of requests to the steam summary handler in milliseconds.")
                .build()
        });

        STEAM_SUMMARY_ERRORS_TOTAL.get_or_init(|| {
            meter.u64_counter("steam_summary_errors_total")
                .with_description("The total number of failed requests to the steam summary handler.")
                .build()
        });

        STEAM_GAME_TIME_TOTAL.get_or_init(|| {
            meter.u64_counter("steam_game_time_total")
                .with_description("The total time in seconds spent playing a game.")
                .build()
        });

        println!("Initialized Steam metrics (poll_interval: {:?}, max_retries: {})",
                 self.poll_interval, self.max_retries);
    }

    async fn jobs(&self) -> Vec<Job> {
        if !self.enabled {
            return vec![];
        }

        // TODO: Create jobs using self.poll_interval, self.steam_ids, etc.
        vec![]
    }
}

fn create_builder() -> Box<dyn ModuleBuilder> {
    Box::new(SteamModuleBuilder::new())
}

inventory::submit! {
    ModuleRegistration {
        name: "steam",
        builder_factory: create_builder,
    }
}