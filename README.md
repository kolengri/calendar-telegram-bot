# Telegram Calendar Bot

A Telegram bot that connects to different calendar providers like iCloud and Google Calendar. Users can add, delete, and retrieve events using natural language inputs without exact commands.

## Project Structure

```
telegram_calendar_bot
├── Cargo.toml
├── README.md
├── src
│ ├── main.rs
│ ├── bot.rs
│ ├── commands
│ │ ├── add_event.rs
│ │ ├── delete_event.rs
│ │ └── mod.rs
│ ├── models
│ │ ├── mod.rs
│ │ ├── event.rs
│ │ └── provider.rs
│ ├── providers
│ │ ├── mod.rs
│ │ ├── google_calendar.rs
│ │ └── icloud_calendar.rs
│ ├── utils
│ │ └── mod.rs
```

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) and Cargo (the Rust package manager)
- API keys and credentials for the calendar providers you want to use (e.g., Google Calendar, iCloud)

## Setup

1. Clone this repository:

```bash
git clone https://github.com/kolegri/telegram_calendar_bot.git
```

2. Change the directory to the cloned repository:

```bash
cd telegram_calendar_bot
```

3. Add your Telegram bot token to `.env` file:

```
TELEGRAM_BOT_TOKEN=your_bot_token
```

4. Build the project:

```bash
cargo build
```

5. Run the Telegram bot:

```bash
cargo run
```

## Usage

Interact with the bot using natural language inputs to add, delete, or retrieve events from your connected calendar providers. The bot will parse your messages and perform the appropriate action based on your input.

For example:

- "Add a meeting with John tomorrow at 3pm"
- "Remove the event on March 15th at 10am"
- "What's on my calendar next week?"

## License

This project is licensed under the [MIT License](LICENSE).
