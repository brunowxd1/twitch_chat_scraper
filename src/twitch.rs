use dotenv::dotenv;
use irc::client::prelude::*;
use std::env;

pub async fn connect_to_twitch() -> irc::error::Result<Client> {
    dotenv().ok();

    let twitch_nickname = env::var("TWITCH_NICKNAME").unwrap();
    let twitch_oauth = env::var("TWITCH_OAUTH").unwrap();
    let twitch_channel = env::var("TWITCH_CHANNEL").unwrap();

    let irc_config = Config {
        nickname: Some(twitch_nickname),
        server: Some("irc.chat.twitch.tv".to_owned()),
        port: Some(6667),
        password: Some(twitch_oauth),
        use_tls: Some(false),
        ping_timeout: Some(10),
        ping_time: Some(10),
        channels: vec![twitch_channel],

        ..Default::default()
    };

    return Client::from_config(irc_config).await;
}
