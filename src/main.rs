use poise::serenity_prelude as serenity;
use sqlx::sqlite::SqlitePool;

struct Data {
    pool: SqlitePool,
} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Displays your or another user's account creation date
#[poise::command(slash_command)]
async fn join(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let pool = &ctx.data().pool;
    let channel_id = ctx.channel_id().0 as i64;
    let channel = sqlx::query!("SELECT * FROM Channels WHERE ChannelId = ?", channel_id)
        .fetch_optional(pool)
        .await;
    if let Ok(Some(_)) = channel {
        ctx.say("Channel already in database").await?;
        return Ok(());
    }
    sqlx::query!("INSERT INTO Channels VALUES (?)", channel_id)
        .execute(pool)
        .await
        .expect("Failed to insert channel into database");
    ctx.say("Channel added to database").await?;
    Ok(())
}

async fn all_event_handler(
    event: &poise::event::Event<'_>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        poise::event::Event::Message { new_message } => {
            println!("message created");
            let msg = new_message;
            let pool = &data.pool;
            let channels = sqlx::query!("SELECT * FROM Channels")
                .fetch_all(pool)
                .await
                .expect("Failed to fetch channels from database");
            for channel in channels {
                let channel_id = channel.ChannelId.unwrap() as u64;
                println!("channel: {}", channel_id);
                if channel_id == msg.channel_id.0 {
                    println!("channel found");
                    continue;
                }
            }
        }
        _ => {}
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let pool = SqlitePool::connect((std::env::var("DATABASE_URL").unwrap()).as_str())
        .await
        .unwrap();
    sqlx::migrate!().run(&pool).await.unwrap();
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![join()],
            event_handler: | ctx, event, _framework, data | {
                Box::pin(all_event_handler(event, data))
            },
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    pool: pool.clone(),
                })
            })
        });
    framework.run().await.unwrap();
}