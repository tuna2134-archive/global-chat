# global-chat
Light weight discord global-chat bot

## Require
- Rust
- Discord message content intents

## 1. Build
```sh
SQLX_OFFLINE=true cargo build --release
```

## 2. Make database file
```sh
touch main.db
```

## 3. Make dotenv file
```
DATABASE_URL=sqlite:main.db
DISCORD_TOKEN=<Your discord bot token>
```

## 4. Run the bot
```
./target/release/global-chat
```

## Command

- `/join` - Join to GlobalChat
