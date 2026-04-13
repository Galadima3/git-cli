use std::collections::HashMap;

use reqwest::Client;
use serde::Deserialize;

use crate::model::WelcomeElement;

mod model;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = Client::new();
    let url = "https://api.github.com/users/Galadima3/events";

    let response = client
        .get(url)
        .header("User-Agent", "rust-client")
        .send()
        .await?;

    if !response.status().is_success() {
        eprintln!("Request failed with status: {}", response.status());
        return Ok(());
    }

    let events = response.json::<Vec<WelcomeElement>>().await?;
    let mut grouped: HashMap<String, HashMap<String, u32>> = HashMap::new();

    

for e in &events {
    let repo = e.repo.name.clone();
    let event_type = e.welcome_type.clone();

    let repo_map = grouped.entry(repo).or_insert_with(HashMap::new);
    *repo_map.entry(event_type).or_insert(0) += 1;
}

for (repo, events) in grouped {
    println!("\n{}", repo);

    for (event_type, count) in events {
        let line = match event_type.as_str() {
            "PushEvent" => format!("- Pushes: {}", count),
            "CreateEvent" => format!("- Created: {}", count),
            "WatchEvent" => format!("- Starred: {}", count),
            "IssuesEvent" => format!("- Issues: {}", count),
            "ForkEvent" => format!("- Forks: {}", count),
            _ => format!("- {}: {}", event_type, count),
        };

        println!("{}", line);
    }
}


    Ok(())
}

