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
