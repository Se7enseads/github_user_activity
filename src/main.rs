fn fetch_github_activity(username: &String) -> Result<(), String> {
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

    // TODO: Parse and format the JSON response for better readability
    let body = response
        .text()
        .map_err(|e| format!("Failed to read response body: {e}"))?;

    // Display the fetched activity
    println!("GitHub Activity for {username}:\n {body}");
    Ok(())
}

fn main() -> Result<(), String> {
    // Get username from command line arguments
    let args = std::env::args().nth(1).unwrap_or_default();
    fetch_github_activity(&args)
}
