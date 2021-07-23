use std::{
    collections::{HashMap, HashSet},
    env,
    sync::Arc,
};

use serenity::prelude::*;
use serenity::{
    async_trait,
    client::bridge::gateway::{ShardId, ShardManager},
    framework::standard::{
        buckets::{LimitedFor, RevertBucket},
        help_commands,
        macros::{check, command, group, help, hook},
        Args,
        CommandGroup,
        CommandOptions,
        CommandResult,
        DispatchError,
        HelpOptions,
        Reason,
        StandardFramework,
    },
    http::Http,
    model::{
        channel::{Channel, Message},
        gateway::Ready,
        id::UserId,
        permissions::Permissions,
    },
    utils::{content_safe, ContentSafeOptions},
};
use tokio::sync::Mutex;


#[group]
#[commands(ping, about)]
struct General;


// A container type is created for inserting into the Client's `data`, which
// allows for data to be accessible across all events and framework commands, or
// anywhere else that has a copy of the `data` Arc.
struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}


struct CommandCounter;

impl TypeMapKey for CommandCounter {
    type Value = HashMap<String, u64>;
}

struct Handler;


#[async_trait]
impl EventHandler for Handler {
    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {

    let token = env::var("DISCORD_BOT_RASIS_TOKEN").expect("token");
    let http = Http::new_with_token(&token);

    // We will fetch your bot's owners and id
    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(team) = info.team {
                owners.insert(team.owner_user_id);
            } else {
                owners.insert(info.owner.id);
            }
            match http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id.id),
                Err(why) => panic!("Could not access the bot id: {:?}", why),
            }
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };


    let framework = StandardFramework::new()            
    .configure(|c| c
        .with_whitespace(true)
        .on_mention(Some(bot_id))
        .delimiters(vec![", ", ","])
        .owners(owners)
        .prefix("~")) // set the bot's prefix to "~"
    .before(before)
    .after(after)
    .normal_message(normal_message)
    .unrecognised_command(unknown_command)
    .group(&GENERAL_GROUP);
    
    // Login with a bot token from the environment
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<CommandCounter>(HashMap::default());
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

// A function which acts as a "check", to determine whether to call a command.
//
// In this case, this command checks to ensure you are the owner of the message
// in order for the command to be executed. If the check fails, the command is
// not called.
#[check]
#[name = "Owner"]
async fn owner_check(
    _: &Context,
    msg: &Message,
    _: &mut Args,
    _: &CommandOptions,
) -> Result<(), Reason> {
    // Replace 7 with your ID to make this check pass.
    //
    // 1. If you want to pass a reason alongside failure you can do:
    // `Reason::User("Lacked admin permission.".to_string())`,
    //
    // 2. If you want to mark it as something you want to log only:
    // `Reason::Log("User lacked admin permission.".to_string())`,
    //
    // 3. If the check's failure origin is unknown you can mark it as such:
    // `Reason::Unknown`
    //
    // 4. If you want log for your system and for the user, use:
    // `Reason::UserAndLog { user, log }`
    if msg.author.id != 7 {
        return Err(Reason::User("Lacked owner permission".to_string()));
    }

    Ok(())
}


#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    println!("received ping!");
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

#[command]
async fn about(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "about").await?;

    Ok(())
}


#[hook]
async fn unknown_command(_ctx: &Context, _msg: &Message, unknown_command_name: &str) {
    println!("Could not find command named '{}'", unknown_command_name);
}

#[hook]
async fn normal_message(_ctx: &Context, msg: &Message) {
    println!("Message is not a command '{}'", msg.content);
}

#[hook]
async fn before(ctx: &Context, msg: &Message, command_name: &str) -> bool {
    println!("Got command '{}' by user '{}'", command_name, msg.author.name);

    // Increment the number of times this command has been run once. If
    // the command's name does not exist in the counter, add a default
    // value of 0.
    let mut data = ctx.data.write().await;
    let counter = data.get_mut::<CommandCounter>().expect("Expected CommandCounter in TypeMap.");
    let entry = counter.entry(command_name.to_string()).or_insert(0);
    *entry += 1;

    true // if `before` returns false, command processing doesn't happen.
}

#[hook]
async fn after(_ctx: &Context, _msg: &Message, command_name: &str, command_result: CommandResult) {
    match command_result {
        Ok(()) => println!("Processed command '{}'", command_name),
        Err(why) => println!("Command '{}' returned error {:?}", command_name, why),
    }
}