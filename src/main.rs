use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::TwitchIRCClient;
use twitch_irc::{ClientConfig, SecureTCPTransport};
use twitch_irc::message::ServerMessage;
use dotenv::dotenv;
use std::env;

/*
    NOTE: This project is a mess, because I have literally zero idea on how to code
    in Rust. Just take the good stuff and fix the messy things lul
*/

#[tokio::main]
pub async fn main() {

    // Load all lines from .env
    dotenv().ok();

    // Temp variables for loading .env
    let mut fetch_login_name = String::from("");
    let mut fetch_oauth = String::from("");

    // Read env keys
    match env::var("TWITCH_LOGIN_NAME") {
        Ok(value) => {
            println!("TWITCH_LOGIN_NAME value is: {}", value);
            fetch_login_name.push_str(value.as_str());
        }
        Err(_) => {
            eprintln!("TWITCH_LOGIN_NAME is not set or is an invalid UTF-8 string.");
        }
    }
    match env::var("TWITCH_OAUTH_TOKEN") {
        Ok(value) => {
            println!("TWITCH_OAUTH_TOKEN value is: {}", value);
            fetch_oauth.push_str(value.as_str());
        }
        Err(_) => {
            eprintln!("TWITCH_OAUTH_TOKEN is not set or is an invalid UTF-8 string");
        }
    }

    // default configuration is to join chat as anonymous.
    //let config = ClientConfig::default();

    // Login with oauth + display name
    let login_name = fetch_login_name.to_owned();
    let oauth_token = fetch_oauth.to_owned();
    let config = ClientConfig::new_simple(
        StaticLoginCredentials::new(login_name, Some(oauth_token))
    );

    // uuh wtf is this
    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    // join a channel
    // This function only returns an error if the passed channel login name is malformed,
    // so in this simple case where the channel name is hardcoded we can ignore the potential
    // error with `unwrap`.
    // NOTE: Remember to add a channel to join!
    let join_channel = "".to_owned();
    client.join(join_channel.to_owned()).unwrap();

    // Send a message when the bot joins
    client.say(join_channel.to_owned(), "omg hii".to_owned()).await.unwrap();

    // first thing you should do: start consuming incoming messages,
    // otherwise they will back up.
    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            match message {
                ServerMessage::Privmsg(msg) => {
                    //println!("(#{}) {}: {}", msg.channel_login, msg.sender.name, msg.message_text);

                    // Logic removed for now, but you can add your own here :3
                    // Here's a couple of useful things
                    // msg.message_text -> get message
                    // msg.message_text.contains("substring")
                    // msg.message_text.starts_with("!cmd")
                    // client.say(join_channel.to_owned(), "message".to_owned()).await.unwrap();
                },
                ServerMessage::Whisper(msg) => {
                    println!("(w) {}: {}", msg.sender.name, msg.message_text);
                },
                _ => {}
            }
        }
    });

    // keep the tokio executor alive.
    // If you return instead of waiting the background task will exit.
    join_handle.await.unwrap();
}
