# 🐙 GitHub Activity CLI

A fast, minimal command-line tool written in **Rust** that fetches and displays a GitHub user's recent activity — grouped by repository and event type — directly in your terminal.

> Project inspired by [roadmap.sh](https://roadmap.sh/projects/github-user-activity)

---

## ✨ Features

- 🔍 Look up any public GitHub user's recent events
- 📂 Activity grouped neatly by repository
- 🏷️ Human-readable event labels (Pushes, Forks, Stars, Issues, etc.)
- 🔁 Interactive REPL loop — query multiple users in one session
- ⚡ Async HTTP via `reqwest` + `tokio`
- 🛡️ Graceful error handling — bad requests don't crash the session

---

## 📦 Installation

**Prerequisites:** [Rust](https://www.rust-lang.org/tools/install) (1.70+)

```bash
git clone https://github.com/Galadima3/git_cli
cd git_cli
cargo build --release
```

---

## 🚀 Usage

```bash
cargo run
```

You'll be dropped into an interactive prompt:

```
Enter GitHub username > torvalds

torvalds/linux
- Pushes: 3
- Issues: 1

torvalds/uemacs
- Pushes: 1
```

Type another username to query again, or press `Ctrl+C` to exit.

---

## 🗂️ Project Structure

```
src/
├── main.rs       # Entry point, REPL loop, display logic
└── model.rs      # Deserialization types for GitHub Events API
```

---

## 🔧 Dependencies

| Crate | Purpose |
|-------|---------|
| `tokio` | Async runtime |
| `reqwest` | HTTP client |
| `serde` / `serde_json` | JSON deserialization |

---

## 📡 API Reference

Uses the public [GitHub Events API](https://docs.github.com/en/rest/activity/events):

```
GET https://api.github.com/users/{username}/events
```

No authentication required for public data (rate limit: 60 req/hr per IP).

---

## 🎯 Supported Event Types

| GitHub Event | Displayed As |
|---|---|
| `PushEvent` | Pushes |
| `CreateEvent` | Created |
| `WatchEvent` | Starred |
| `IssuesEvent` | Issues |
| `ForkEvent` | Forks |
| anything else | Shown as-is |

---

## 📄 License

MIT — do whatever you want with it.
