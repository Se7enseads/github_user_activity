# Github User Activity

This Rust project fetches and displays the recent activity of a specified GitHub user. It utilizes the GitHub API to retrieve information about the user's events.
From [Roadmap.sh](https://roadmap.sh/projects/github-user-activity)

```text
https://api.github.com/users/{username}/events
# Example: https://api.github.com/users/kamranahmedse/events
```

Specify the GitHub username as a command-line argument when running the program.

```bash
cargo run -- <github_username>
```

Displays the recent activity of the specified GitHub user in the terminal.

```text
Output:
- Pushed 3 commits to kamranahmedse/developer-roadmap
- Opened a new issue in kamranahmedse/developer-roadmap
- Starred kamranahmedse/developer-roadmap
- ...
```
