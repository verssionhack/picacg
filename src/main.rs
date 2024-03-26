#![allow(unused)]

use clap::Parser;
use client::Client;
use console::Console;

use picacg::command::{ComicOptions, GameOptions, GlobalOptions, SubCommand, UserOptions};
use reqwest::Proxy;
use std::{
    io::{stdout, Write, stdin},
};

mod client;
mod console;

mod handle {
    use crate::client::Client;
    pub mod comic {
        use libpicacg::{Sort};
        use picacg::{console::Console, command::GlobalOptions};

        use super::*;
        pub async fn ranking(client: &mut Client, options: &GlobalOptions) {
            match client.comic_ranking().await {
                Ok(res) => {
                    for comic in res.iter() {
                        println!("{}", Console::format_comic(&comic));
                        if options.download {
                            client.comic_download_eps(&comic.id, &options.save_dir).await.map_err(|err| {
                                Console::clear_line();
                                println!("{}", Console::format_error(&err));
                            });
                        }
                    }
                }
                Err(err) => {
                    println!("{}", Console::format_error(&err))
                }
            }
        }
        pub async fn metadata(client: &mut Client, options: &GlobalOptions, cids: Vec<String>, _save_dir: Option<String>) {
            for cid in cids {
                match client.comic_metadata(&cid).await {
                    Ok(res) => {
                        println!("{}", Console::format_comic_metadata(&res));
                        if options.download {
                            client.comic_download_eps(&res.metadata.id, &options.save_dir).await.map_err(|err| {
                                Console::clear_line();
                                println!("{}", Console::format_error(&err));
                            });
                        }
                    }
                    Err(err) => {
                        println!("{}", Console::format_error(&err))
                    }
                }
            }
        }
        pub async fn recommended(client: &mut Client, options: &GlobalOptions, cids: Vec<String>, _save_dir: Option<String>) {
            for cid in cids {
                match client.comic_recommended(&cid).await {
                    Ok(res) => {
                        for comic in res.iter() {
                            println!("{}", Console::format_comic(&comic));
                            if options.download {
                                client.comic_download_eps(&comic.id, &options.save_dir).await.map_err(|err| {
                                    Console::clear_line();
                                    println!("{}", Console::format_error(&err));
                                });
                            }
                        }
                    }
                    Err(err) => {
                        println!("{}", Console::format_error(&err))
                    }
                }
            }
        }
        pub async fn eps(
            client: &mut Client,
            options: &GlobalOptions,
            cid: String,
            start: u64,
            end: Option<u64>,
            _save_dir: Option<String>,
        ) {
            let mut end = end.unwrap_or(start);
            for page in start..=end {
                match client.comic_eps(&cid, page).await {
                    Ok(res) => {
                        if options.until_end {
                            end = res.pages;
                        }
                        for ep in res.iter() {
                            println!("{}", Console::format_ep(ep));
                        }
                        if options.download {
                            client.comic_download_eps(&cid, &options.save_dir).await.map_err(|err| {
                                Console::clear_line();
                                println!("{}", Console::format_error(&err));
                            });
                        }
                    }
                    Err(err) => {
                        println!("{}", Console::format_error(&err))
                    }
                }
            }
        }
        pub async fn pages(
            client: &mut Client,
            options: &GlobalOptions,
            cid: String,
            start_index: u64,
            _end_index: Option<u64>,
            start: u64,
            end: Option<u64>,
            _save_dir: Option<String>,
        ) {
            let mut end_index = end.unwrap_or(start_index);
            for page_index in start_index..=end_index {
                let mut end = end.unwrap_or(start);
                for page in start..=end {
                    match client.comic_pages(&cid, page_index, page).await {
                        Ok(res) => {
                            if options.until_end {
                                end = res.pages;
                            }
                            for page in res.iter() {
                                println!("{}", Console::format_page(page));
                            }
                            if options.download {
                                client.comic_download_eps(&cid, &options.save_dir).await.map_err(|err| {
                                    Console::clear_line();
                                    println!("{}", Console::format_error(&err));
                                });
                            }
                        }
                        Err(err) => {
                            println!("{}", Console::format_error(&err))
                        }
                    }
                }
            }
        }
        pub async fn pic_like_get(
            client: &mut Client,
            options: &GlobalOptions,
            cid: String,
            start: u64,
            end: Option<u64>,
            _save_dir: Option<String>,
        ) {
            let mut end = end.unwrap_or(start);
            for page in start..=end {
                match client.pic_like_get(&cid, page).await {
                    Ok(res) => {
                        for comic in res.iter() {
                            println!("{}", Console::format_recommend_pic_like(comic));
                        }
                        if options.download {
                            client.comic_download_eps(&cid, &options.save_dir).await.map_err(|err| {
                                Console::clear_line();
                                println!("{}", Console::format_error(&err));
                            });
                        }
                    }
                    Err(err) => {
                        println!("{}", Console::format_error(&err))
                    }
                }
            }
        }
        pub async fn search(
            client: &mut Client,
            options: &GlobalOptions,
            keyword: String,
            start: u64,
            end: Option<u64>,
            _save_dir: Option<String>,
        ) {
            let mut end = end.unwrap_or(start);
            for page in start..=end {
                match client.search(&keyword, page, Sort::MaxLike).await {
                    Ok(res) => {
                        if options.until_end {
                            end = res.pages;
                        }
                        for row in res.iter() {
                            println!("{}", Console::format_searchrow(row));
                            if options.download {
                                client.comic_download_eps(&row.id, &options.save_dir).await.map_err(|err| {
                                    Console::clear_line();
                                    println!("{}", Console::format_error(&err));
                                });
                            }
                        }
                    }
                    Err(err) => {
                        println!("{}", Console::format_error(&err))
                    }
                }
            }
        }
        pub async fn favourites(
            client: &mut Client,
            options: &GlobalOptions,
            start: u64,
            end: Option<u64>,
            _save_dir: Option<String>,
        ) {
            let mut end = end.unwrap_or(start);
            for page in start..=end {
                match client.favorites(page, Sort::DescByDate).await {
                    Ok(res) => {
                        if options.until_end {
                            end = res.pages;
                        }
                        for comic in res.iter() {
                            println!("{}", Console::format_comic(comic));
                            if options.download {
                                client.comic_download_eps(&comic.id, &options.save_dir).await.map_err(|err| {
                                    Console::clear_line();
                                    println!("{}", Console::format_error(&err));
                                });
                            }
                        }
                    }
                    Err(err) => {
                        println!("{}", Console::format_error(&err))
                    }
                }
            }
        }
        pub async fn download(client: &mut Client, options: &GlobalOptions, cids: Vec<String>, save_dir: Option<String>) {
            let save_dir = options.save_dir.clone() + "/" + &save_dir.unwrap_or(".".to_owned());

            for cid in &cids {
                match client.comic_download_eps(cid, &save_dir).await {
                    Ok(()) => {}
                    Err(err) => {
                        println!("{}", Console::format_error(&err))
                    }
                }
            }
        }
    }
    pub mod game {
        use picacg::{console::Console, command::GlobalOptions};

        use super::*;

        pub async fn games(
            client: &mut Client,
            options: &GlobalOptions,
            start: u64,
            end: Option<u64>,
            _save_dir: Option<String>,
        ) {
            let end = end.unwrap_or(start);
            for page in start..=end {
                match client.games(page).await {
                    Ok(res) => {
                        for game in res.iter() {
                            println!("{}", Console::format_game(game));
                            if options.download {
                                client.game_download(&game.id, &options.save_dir).await.map_err(|err| {
                                    Console::clear_line();
                                    println!("{}", Console::format_error(&err));
                                });
                            }
                        }
                    }
                    Err(err) => {
                        println!("{}", Console::format_error(&err))
                    }
                }
            }
        }
        pub async fn info(client: &mut Client, options: &GlobalOptions, cids: Vec<String>, _save_dir: Option<String>) {
            for cid in cids {
                match client.game_info(&cid).await {
                    Ok(res) => {
                        println!("{}", Console::format_game_info(&res));
                        if options.download {
                            client.game_download(&res.id, &options.save_dir).await.map_err(|err| {
                                Console::clear_line();
                                println!("{}", Console::format_error(&err));
                            });
                        }
                    }
                    Err(err) => {
                        println!("{}", Console::format_error(&err))
                    }
                }
            }
        }
        pub async fn download(client: &mut Client, options: &GlobalOptions, cids: Vec<String>, save_dir: Option<String>) {
            let save_dir = options.save_dir.clone() + "/" + &save_dir.unwrap_or(".".to_owned());

            for cid in cids {
                match client.game_download(&cid, &save_dir).await {
                    Ok(_res) => {}
                    Err(err) => {
                        println!("{}", Console::format_error(&err))
                    }
                }
            }
        }
    }
    pub mod user {
        use picacg::console::Console;

        use super::*;

        pub async fn punch_in(client: &mut Client) {
            match client.punch_in().await {
                Ok(res) => {
                    println!("{}", Console::format_punch_in(&res));
                }
                Err(err) => {
                    println!("{}", Console::format_error(&err))
                }
            }
        }
        pub async fn profile(client: &mut Client) {
            match client.profile().await {
                Ok(res) => {
                    println!("{}", Console::format_profile(&res));
                }
                Err(err) => {
                    println!("{}", Console::format_error(&err))
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let options = GlobalOptions::parse();
    let mut client = Client::new();
    if let Some(ref v) = options.all_proxy {
        client.set_proxy(Some(Proxy::all(v).unwrap())).unwrap();
    } else if let Some(ref v) = options.https_proxy {
        client.set_proxy(Some(Proxy::https(v).unwrap())).unwrap();
    } else if let Some(ref v) = options.http_proxy {
        client.set_proxy(Some(Proxy::http(v).unwrap())).unwrap();
    }

    let mut user = String::new();
    let mut passwd = String::new();

    let std_in = stdin();
    let mut std_out = stdout();

    print!("User: ");
    std_out.flush().unwrap();
    std_in.read_line(&mut user).unwrap();
    print!("Password: ");
    std_out.flush().unwrap();
    std_in.read_line(&mut passwd).unwrap();

    if let Err(err) = client.login(&user[..user.len() - 1], &passwd[..passwd.len() - 1]).await {
        println!("{}", Console::format_error(&err));
        return;
    }

    match options.subcommand.clone() {
        SubCommand::Comic(opts) => match opts {
            ComicOptions::Ranking => {
                handle::comic::ranking(&mut client, &options).await;
            }
            ComicOptions::Metadata { cids, save_dir } => {
                handle::comic::metadata(&mut client, &options, cids, save_dir).await;
            }
            ComicOptions::Recommended { cids, save_dir } => {
                handle::comic::recommended(&mut client, &options, cids, save_dir).await;
            }
            ComicOptions::Eps {
                cid,
                start,
                end,
                save_dir,
            } => {
                handle::comic::eps(&mut client, &options, cid, start, end, save_dir).await;
            }
            ComicOptions::Pages {
                cid,
                start_index,
                end_index,
                start,
                end,
                save_dir,
            } => {
                handle::comic::pages(
                    &mut client,
                    &options,
                    cid,
                    start_index,
                    end_index,
                    start,
                    end,
                    save_dir,
                )
                .await;
            }
            ComicOptions::PicLikeGet {
                cid,
                start,
                end,
                save_dir,
            } => {
                handle::comic::pic_like_get(&mut client, &options, cid, start, end, save_dir).await;
            }
            ComicOptions::Search {
                keyword,
                start,
                end,
                save_dir,
            } => {
                handle::comic::search(&mut client, &options, keyword, start, end, save_dir).await;
            }

            ComicOptions::Favourites {
                start,
                end,
                save_dir,
            } => {
                handle::comic::favourites(&mut client, &options, start, end, save_dir).await;
            }
            ComicOptions::Download { cids, save_dir } => {
                handle::comic::download(&mut client, &options, cids, save_dir).await;
            }
        },
        SubCommand::Game(opts) => match opts {
            GameOptions::Games {
                start,
                end,
                save_dir,
            } => {
                handle::game::games(&mut client, &options, start, end, save_dir).await;
            }
            GameOptions::Info { cids, save_dir } => {
                handle::game::info(&mut client, &options, cids, save_dir).await;
            }
            GameOptions::Download { cids, save_dir } => {
                handle::game::download(&mut client, &options, cids, save_dir).await;
            }
        },
        SubCommand::User(opts) => match opts {
            UserOptions::PunchIn => {
                handle::user::punch_in(&mut client).await;
            }
            UserOptions::Profile => {
                handle::user::profile(&mut client).await;
            }
        },
    }
}
