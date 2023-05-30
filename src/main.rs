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
    sqlx::query!("INSERT INTO Channels VALUES (?)", channel_id)
        .execute(pool)
        .await
        .expect("Failed to insert channel into database");
    ctx.say("Channel added to database").await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let pool = SqlitePool::connect((std::env::var("DATABASE_URL").unwrap()).as_str())
        .await
        .unwrap();
    sqlx::migrate!().run(&pool).await.unwrap();
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![join()],
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