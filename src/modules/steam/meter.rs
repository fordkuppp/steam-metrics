// use lazy_static::lazy_static;
// use opentelemetry::{
//     metrics::{Counter, Meter, Histogram, Gauge},
//     global,
// };
// 
// lazy_static! {
//     static ref METER: Meter = global::meter("my-app");
// 
//     pub static ref STEAM_SUMMARY_LATENCY: Histogram<f64> = METER
//         .f64_histogram("steam_summary_latency")
//         .with_description("The duration of requests to the steam summary handler in milliseconds.")
//         .build();
// 
//     pub static ref STEAM_SUMMARY_ERRORS_TOTAL: Counter<u64> = METER
//         .u64_counter("steam_summary_errors_total")
//         .with_description("The total number of failed requests to the steam summary handler.")
//         .build();
// 
//     pub static ref STEAM_GAME_TIME_TOTAL: Counter<u64> = METER
//         .u64_counter("steam_game_time_total")
//         .with_description("The total time in seconds spent playing a game.")
//         .build();
// }