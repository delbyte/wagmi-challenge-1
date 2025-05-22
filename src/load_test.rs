use futures::{stream, StreamExt};
use reqwest::Client;
use std::time::{Duration, Instant};
use tokio::time::sleep;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .pool_max_idle_per_host(0)
        .timeout(Duration::from_secs(10))
        .build()?;
    
    let url = "https://wagmi-challenge-1-production.up.railway.app/wagmi";
    let total_requests = 10_000;
    let concurrency = 2_500;
    
    println!("Starting load test with {} requests at concurrency {}", total_requests, concurrency);
    let start = Instant::now();
    
    sleep(Duration::from_millis(100)).await;
    
    let results = stream::iter(0..total_requests)
        .map(|i| {
            let client = client.clone();
            let url = url.to_string();
            
            async move {
                let request_start = Instant::now();
                let result = client.post(&url)
                    .header("Content-Type", "application/json")
                    .body(r#"{"a": 5, "b": 10}"#)
                    .send()
                    .await;
                
                let status = match &result {
                    Ok(resp) => resp.status().as_u16(),
                    Err(_) => 0,
                };
                
                let duration = request_start.elapsed();
                (i, status, duration, result.is_ok())
            }
        })
        .buffer_unordered(concurrency)
        .collect::<Vec<_>>()
        .await;
    
    let total_duration = start.elapsed();
    
    // Process results
    let success_count = results.iter().filter(|(_, _, _, success)| *success).count();
    let failure_count = results.len() - success_count;
    
    let durations: Vec<Duration> = results.iter()
        .map(|(_, _, duration, _)| *duration)
        .collect();
    
    let avg_duration = durations.iter()
        .sum::<Duration>()
        .as_secs_f64() / durations.len() as f64;
    
    let mut sorted_durations = durations.clone();
    sorted_durations.sort();
    
    let p50 = percentile(&sorted_durations, 0.5);
    let p90 = percentile(&sorted_durations, 0.9);
    let p99 = percentile(&sorted_durations, 0.99);
    
    println!("\nResults:");
    println!("Total requests: {}", total_requests);
    println!("Successful requests: {}", success_count);
    println!("Failed requests: {}", failure_count);
    println!("Total time: {:.2?}", total_duration);
    println!("Requests per second: {:.2}", total_requests as f64 / total_duration.as_secs_f64());
    println!("\nLatency statistics:");
    println!("Average: {:.2?}", Duration::from_secs_f64(avg_duration));
    println!("50th percentile (median): {:.2?}", p50);
    println!("90th percentile: {:.2?}", p90);
    println!("99th percentile: {:.2?}", p99);
    
    let mut status_counts = std::collections::HashMap::new();
    for (_, status, _, _) in &results {
        *status_counts.entry(*status).or_insert(0) += 1;
    }
    
    println!("\nStatus code distribution:");
    for (status, count) in status_counts {
        let status_text = if status == 0 {
            "Connection Error".to_string()
        } else {
            format!("{}", status)
        };
        println!("{}: {} ({:.2}%)", status_text, count, 100.0 * count as f64 / total_requests as f64);
    }
    
    Ok(())
}

fn percentile(sorted_durations: &[Duration], percentile: f64) -> Duration {
    if sorted_durations.is_empty() {
        return Duration::from_secs(0);
    }
    
    let index = (sorted_durations.len() as f64 * percentile) as usize;
    let index = index.min(sorted_durations.len() - 1);
    sorted_durations[index]
}