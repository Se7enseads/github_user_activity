#[derive(serde::Deserialize, Debug)]
struct Event {
    r#type: EventType,
    payload: Payload,
    repo: Repo,
    created_at: String,
}

#[derive(serde::Deserialize, Debug)]
struct Payload {
    action: Option<String>,
    issue: Option<Issue>,
    head: Option<String>,
    before: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
struct CompareResponse {
    total_commits: usize,
}

#[derive(serde::Deserialize, Debug)]
struct Repo {
    name: String,
}

#[derive(serde::Deserialize, Debug)]
struct Issue {
    url: String,
}

#[derive(serde::Deserialize, Debug)]
enum EventType {
    CommitCommentEvent,
    CreateEvent,
    DeleteEvent,
    DiscussionEvent,
    ForkEvent,
    GollumEvent,
    IssueCommentEvent,
    IssuesEvent,
    MemberEvent,
    PublicEvent,
    PullRequestEvent,
    PullRequestReviewEvent,
    PullRequestReviewCommentEvent,
    PushEvent,
    ReleaseEvent,
    WatchEvent,
}

fn display_user_activity(events: &Vec<Event>) {
    for event in events {
        match event.r#type {
            EventType::CreateEvent => {
                println!("Created {}", event.repo.name);
            }
            EventType::DeleteEvent => {
                println!("Deleted {}", event.repo.name);
            }
            EventType::ForkEvent => {
                println!("Forked {}", event.repo.name);
            }
            EventType::PushEvent => {
                // Try to get commit count from the compare API
                let commit_count = if let (Some(before), Some(head)) =
                    (&event.payload.before, &event.payload.head)
                {
                    fetch_commit_count(&event.repo.name, before, head).unwrap_or(0)
                } else {
                    0
                };

                if commit_count > 0 {
                    println!("Pushed {} commit(s) to {}", commit_count, event.repo.name);
                } else {
                    println!("Pushed to {}", event.repo.name);
                }
                println!("Pushed to {}", event.repo.name);
            }
            EventType::WatchEvent => {
                println!("Starred {}", event.repo.name);
            }
            EventType::IssueCommentEvent => {
                if let Some(issue) = &event.payload.issue {
                    println!("Commented on {}", issue.url);
                }
            }
            EventType::IssuesEvent => {
                if let Some(action) = &event.payload.action {
                    println!("{} issue on {}", action, event.repo.name);
                }
            }
            EventType::PullRequestEvent => {
                if let Some(action) = &event.payload.action {
                    println!("{} pull request on {}", action, event.repo.name);
                }
            }
            EventType::CommitCommentEvent => {
                if let Some(action) = &event.payload.action {
                    println!("{} {}", action, event.repo.name);
                }
            }

            _ => {
                println!(
                    "[{:?}] on {} at {} {}",
                    event.r#type,
                    event.repo.name,
                    event.created_at,
                    event.payload.action.as_deref().unwrap_or("")
                );
            }
        }
    }
}

fn fetch_commit_count(repo: &str, before: &str, head: &str) -> Result<usize, String> {
    let url = format!(
        "https://api.github.com/repos/{}/compare/{}...{}",
        repo, before, head
    );

    let client = reqwest::blocking::Client::new();
    let response = client
        .get(&url)
        .header("Accept", "application/vnd.github.v3+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "Se7enseads")
        .send()
        .map_err(|e| format!("Failed to send request: {e}"))?;

    if !response.status().is_success() {
        return Err(format!("Failed to fetch count: HTTP {}", response.status()));
    }

    let compare_response = response
        .json::<CompareResponse>()
        .map_err(|e| format!("Failed to read response body: {e}"))?;

    Ok(compare_response.total_commits)
}

fn fetch_github_activity(username: &String) -> Result<Vec<Event>, String> {
    // Check if username is empty
    if username.is_empty() {
        return Err("Username cannot be empty".to_string());
    }

    // Construct the API URL with the provided username
    let url = format!("https://api.github.com/users/{}/events", username);

    // Fetch data from GitHub API
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(&url)
        .header("Accept", "application/vnd.github.v3+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "Se7enseads")
        .send()
        .map_err(|e| format!("Failed to send request: {e}"))?;

    // Handle rate limiting
    if response.status() == reqwest::StatusCode::FORBIDDEN {
        let time = response
            .headers()
            .get("X-RateLimit-Reset")
            .and_then(|value| value.to_str().ok())
            .and_then(|s| s.parse::<u64>().ok())
            .map(|timestamp| {
                let datetime = chrono::DateTime::from_timestamp(timestamp as i64, 0);
                datetime
                    .map(|dt| dt.format("%H:%M:%S").to_string())
                    .unwrap_or_else(|| "unknown time".to_string())
            })
            .unwrap_or_else(|| "unknown time".to_string());
        let error = format!("Rate limit exceeded. Resets at {time}");
        return Err(error);
    }

    // Return error if response status is not success
    if !response.status().is_success() {
        return Err(format!(
            "Failed to fetch github activity: HTTP {}",
            response.status()
        ));
    }

    // Return the JSON response
    let json = response
        .json::<Vec<Event>>()
        .map_err(|e| format!("Failed to read response body: {e}"));
    json
}

fn main() {
    // Get username from command line arguments
    let args = std::env::args().nth(1).unwrap_or_default();
    let username = args.trim().to_string();

    match fetch_github_activity(&username) {
        Ok(events) => {
            display_user_activity(&events);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
