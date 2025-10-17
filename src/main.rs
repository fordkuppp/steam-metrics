extern crate dotenv;

use std::env;
use std::sync::Arc;
use crate::trackers::steam::client::SteamClient;
use axum::{Router, routing::get};
use dotenv::dotenv;
use opentelemetry::{global, KeyValue};
use opentelemetry_sdk::metrics::SdkMeterProvider;
use tokio_cron_scheduler::{Job, JobScheduler};
use crate::trackers::steam::scheduler::SteamScheduler;
use anyhow::Result;

// use opentelemetry::{
//     global,
//     runtime::Tokio,
//     sdk::{metrics::{AggregationSelector, InstrumentKind, Period}, Resource, trace::self},
// };
// use opentelemetry_otlp::{
//     WithEndpoint,
//     WithMetadata,
//     WithTonicExporter,
// };
// use opentelemetry_semantic_conventions::resource::{
//     SERVICE_NAME, SERVICE_VERSION
// };
//
// mod metrics;
// mod routes;
mod metrics;
mod trackers;
mod modules;

fn init_metrics() -> SdkMeterProvider {
    let stdout_exporter = opentelemetry_stdout::MetricExporter::default();
    let otlp_exporter = opentelemetry_otlp::MetricExporter::builder()
        .with_tonic()
        .build()
        .expect("Failed");
    SdkMeterProvider::builder()
        .with_periodic_exporter(stdout_exporter)
        .with_periodic_exporter(otlp_exporter)
        .build()
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let meter_provider = init_metrics();
    global::set_meter_provider(meter_provider);

    SteamScheduler::new().await?;

    match tokio::signal::ctrl_c().await {
        Ok(()) => {
            println!("Shutting down gracefully...");
        },
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal: {}", err);
        },
    }

    Ok(())
}
