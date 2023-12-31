#![allow(proc_macro_derive_resolution_fallback, unused_attributes)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;

#[macro_use]
extern crate serde_derive;

use anyhow::Result;
use futures::prelude::*;
use irc::client::prelude::*;

use dotenv::dotenv;
use models::NewUser;
use std::time::SystemTime;

use crate::db::init_connection;
use crate::models::NewComment;
use crate::models::User;
use crate::twitch::connect_to_twitch;

mod db;
mod models;
mod schema;
mod twitch;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let connection = init_connection();

    let mut client = connect_to_twitch().await?;
    client.identify()?;

    client.send(Command::CAP(
        None,
        irc::proto::CapSubCommand::REQ,
        Some(":twitch.tv/tags".to_owned()),
        None,
    ))?;

    let mut stream = client.stream()?;

    while let Some(message) = stream.next().await.transpose()? {
        if let Command::PRIVMSG(_, chat_message) = message.command {
            match message.prefix {
                Some(p) => {
                    if let Prefix::Nickname(username, _, _) = p {
                        match message.tags {
                            Some(x) => {
                                let user = User::check_if_user_exists(&username, &connection);

                                if user.len() == 0 {
                                    let badges = x[1].1.as_ref().ok_or("No Badges").unwrap();

                                    let mut is_broadcaster = false;
                                    let mut is_partner = false;
                                    let mut is_vip = false;
                                    let mut is_mod = false;
                                    let mut is_sub = false;
                                    let mut is_admin = false;

                                    if badges.contains("broadcaster") {
                                        is_broadcaster = true;
                                    } else if badges.contains("subscriber") {
                                        is_sub = true;
                                    } else if badges.contains("admin") {
                                        is_admin = true;
                                    } else if badges.contains("vip") {
                                        is_vip = true;
                                    } else if badges.contains("moderator") {
                                        is_mod = true;
                                    } else if badges.contains("partner") {
                                        is_partner = true;
                                    }

                                    let new_user: NewUser = NewUser {
                                        username: (username),
                                        is_sub: (is_sub),
                                        is_partner: (is_partner),
                                        is_mod: (is_mod),
                                        is_vip: (is_vip),
                                        is_admin: (is_admin),
                                        is_broadcaster: (is_broadcaster),
                                    };

                                    let inserted_user = User::insert_user(new_user, &connection);

                                    let new_comment = NewComment {
                                        user_id: inserted_user[0].id,
                                        comment: chat_message,
                                        created_at: SystemTime::now(),
                                    };

                                    NewComment::insert_comment(new_comment, &connection);
                                } else {
                                    let new_comment = NewComment {
                                        user_id: user[0].id,
                                        comment: chat_message,
                                        created_at: SystemTime::now(),
                                    };

                                    NewComment::insert_comment(new_comment, &connection);
                                }
                            }
                            None => println!("No tags"),
                        }
                    }
                }
                None => println!("None"),
            }
        }
    }

    Ok(())
}
