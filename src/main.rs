use serenity::{async_trait};
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::Message;
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};

use songbird::{SerenityInit};

use std::env;

mod joke_database;
mod joke;

#[group]
#[only_in(guilds)]
#[commands(joke, leave)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {

    joke::Joke::init();

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("}"))
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token not found");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .register_songbird()
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
#[only_in(guilds)]
async fn joke(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.expect("No guild found");
    let guild_id = guild.id;
    let file_name = format!("{}_{}_{}", msg.timestamp.clone(), guild.id.to_string(), msg.author.id.to_string());

    let channel_id = guild
        .voice_states.get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id).expect("User not in a channel");
    
    let manager = songbird::get(ctx).await.unwrap().clone();
    let (call, _) = manager.join(guild_id, channel_id).await;

    //let joke = joke::Joke::new("What do computers and air conditioners have in common? <break time=\"3s\"/> They are both useless when you open Windows.".to_string(), &file_name);

    let jdb = joke_database::Joke_Database::new().expect("Could not connect to db");
    let joke_string = jdb.get_random_joke().expect( "Could not find a joke");
    let joke = joke::Joke::new(joke_string, &file_name);

    let joke_input = songbird::ffmpeg(joke.get_joke_file_path()).await.expect("Error getting joke file");
    let handle = call.lock().await.play_source(joke_input);
    
    loop{
        std::thread::sleep(std::time::Duration::from_millis(100));
        let info = handle.get_info().await;

        match info {
            Err(songbird::tracks::TrackError::Finished) => break,
            Err(_) => (),
            Ok(_) => ()
        }
    }

    joke.clean_up_files();

    manager.remove(guild_id).await.expect("Cannot leave voice channel");

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.expect("No guild found");
    let guild_id = guild.id;

    let manager = songbird::get(ctx).await.unwrap().clone();

    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        manager.remove(guild_id).await.expect("Cannot leave voice channel");
    }

    Ok(())
}