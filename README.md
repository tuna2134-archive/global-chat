# global-chat

## Require
- Rust

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
