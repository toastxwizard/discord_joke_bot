use serenity::{async_trait, futures::future::UnsafeFutureObj};
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
use songbird::Call;

use std::{fs::create_dir, io::Write, path, process::{Command, Stdio}};
use std::fs::remove_file;

use std::env;
use std::path::Path;
use std::time;

#[group]
#[only_in(guilds)]
#[commands(joke, leave)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    if !Path::new("jokes").exists() {
        std::fs::create_dir("jokes").unwrap();
    }

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
    let file_name = format!("jokes/{}_{}", msg.timestamp, guild.name);
    let file_name_final = format!("{}_joke.wav", file_name.clone());
    let file_name_conversion = format!("{}.wav", file_name.clone());

    let channel_id = guild
        .voice_states.get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id).expect("User not in a channel");
    
    let manager = songbird::get(ctx).await.unwrap().clone();

    let (call, _) = manager.join(guild_id, channel_id).await;
    
    let espeak_process = Command::new("espeak")
        .arg("--stdin")
        .arg("-w")
        .arg(file_name_conversion.clone())
        .stdin(Stdio::piped())
        .spawn()
        .expect("Error creating buffer file");

    espeak_process.stdin.unwrap()
        .write_all("Boss, you killed a child........................................... That's why you are the boss".as_bytes())
        .expect("Error generating tts file");

    Command::new("ffmpeg")
        .arg("-i")
        .arg(file_name_conversion.clone())
        .arg(file_name_final.clone())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .expect("Error converting to ffmpeg format");

    let joke = songbird::ffmpeg(file_name_final.clone()).await.expect("Could not load soundfile");

    let _handle = call.lock().await.play_source(joke);

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