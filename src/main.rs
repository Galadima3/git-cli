use std::{
    collections::HashMap,
    io::{self, Write},
};

use reqwest::Client;
use crate::model::Welcome;
mod model;

const GITHUB_API_BASE: &str = "https://api.github.com";

#[tokio::main]
async fn main() {
    let client = Client::new();
    loop {
        let input = prompt_username();
        if input.is_empty() {
            continue;
        }
        if let Err(e) = fetch_and_display(&client, &input).await {
            eprintln!("Error: {e}");
        }
    }
}

fn prompt_username() -> String {
    print!("\nEnter GitHub username > ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_owned()
}

async fn fetch_and_display(
    client: &Client,
    username: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{GITHUB_API_BASE}/users/{username}/events");

    let response = client
        .get(&url)
        .header("User-Agent", "rust-client")
        .send()
        .await?;

    if !response.status().is_success() {
        eprintln!("Request failed with status: {}", response.status());
        return Ok(());
    }

    let events: Welcome = response.json().await?;
    let grouped = group_events(&events);
    print_grouped_events(&grouped);

    Ok(())
}

fn group_events(events: &Welcome) -> HashMap<&str, HashMap<&str, u32>> {
    let mut grouped: HashMap<&str, HashMap<&str, u32>> = HashMap::new();

    for event in events {
        let repo_map = grouped.entry(&event.repo.name).or_default();

        *repo_map.entry(&event.welcome_type).or_insert(0) += 1;
    }

    grouped
}

fn format_event(event_type: &str, count: &u32) -> String {
    let label = match event_type {
        "PushEvent" => "Pushes",
        "CreateEvent" => "Created",
        "WatchEvent" => "Starred",
        "IssuesEvent" => "Issues",
        "ForkEvent" => "Forks",
        other => return format!("- {other}: {count}"),
    };
    format!("- {label}: {count}")
}

fn print_grouped_events(grouped: &HashMap<&str, HashMap<&str, u32>>) {
    let mut repos: Vec<_> = grouped.keys().collect();
    repos.sort();

    for repo in repos {
        println!("\n{repo}");

        let mut event_entries: Vec<_> = grouped[repo].iter().collect();
        event_entries.sort_by_key(|(k, _)| *k);

        for (event_type, count) in event_entries {
            println!("{}", format_event(event_type, count));
        }
    }
}
