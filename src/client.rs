#![allow(unused)]

use std::{
    io::{stdout, Write},
    ops::{Deref, DerefMut},
    path::PathBuf,
    str::FromStr,
    sync::Arc,
    time::Duration,
};

use libpicacg::{error::Error, Api, Pagible};
use reqwest::{ClientBuilder, Proxy, RequestBuilder};
use size_utils::Size;
use tokio::{
    fs,
    io::{AsyncReadExt, AsyncWriteExt},
    sync::RwLock,
};

pub fn to_full_width_char(c: char) -> char {
    char::from_u32(c as u32 + '？' as u32 - '?' as u32).unwrap()
}

pub fn path_escape(path: &str) -> String {
    let chars = ['\\', '/', ':', '*', '?', '"', '<', '>', '|'];
    let mut path = path.to_string();
    for c in chars {
        path = path.replace(c, &to_full_width_char(c).to_string());
    }
    path
}

use crate::console::Console;

pub struct Client {
    api: Api,
    client: reqwest::Client,
}

impl Deref for Client {
    type Target = Api;
    fn deref(&self) -> &Self::Target {
        &self.api
    }
}

impl DerefMut for Client {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.api
    }
}

impl Client {
    pub fn new() -> Self {
        Self {
            api: Api::new(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn login(&mut self, email: &str, password: &str) -> Result<(), Error> {
        self.api.login(email, password).await
    }

    pub fn get(&self, url: &str) -> RequestBuilder {
        self.client.get(url)
    }

    pub fn post(&self, url: &str) -> RequestBuilder {
        self.client.post(url)
    }

    fn reset_client(&mut self) -> Result<(), Error> {
        let mut client_builder = ClientBuilder::new();
        if let Some(proxy) = self.api.proxy() {
            client_builder = client_builder.proxy(proxy.clone());
        }
        if let Some(timeout) = self.api.timeout() {
            client_builder = client_builder.timeout(timeout.clone());
        }
        self.client = client_builder.build()?;
        Ok(())
    }

    pub fn set_timeout(&mut self, timeout: Option<Duration>) -> Result<(), Error> {
        self.api.set_timeout(timeout.clone())?;
        self.reset_client()?;
        Ok(())
    }

    pub fn set_proxy(&mut self, proxy: Option<Proxy>) -> Result<(), Error> {
        self.api.set_proxy(proxy.clone())?;
        self.reset_client()?;
        Ok(())
    }

    pub async fn game_download(&self, cid: &str, savedir: &str) -> Result<(), Error> {
        let game_info = self.game_info(cid).await?;
        let output_dir = PathBuf::from_str(savedir).unwrap();
        if !output_dir.exists() {
            fs::create_dir_all(&output_dir).await?;
        }
        let mut file_path = output_dir.join(path_escape(&game_info.title));
        let download_info = self
            .game_download_info_get(&game_info.android_links[0])
            .await?;

        let download_url = &download_info.download.node[0];
        file_path.set_extension(download_url.as_str().rsplit_once('.').unwrap().1);
        let file_path_str = file_path
            .as_path()
            .as_os_str()
            .to_str()
            .unwrap()
            .to_string();

        let request = self
            .client
            .get(download_url)
            .header("referer", &game_info.android_links[0]);

        let request_head = self
            .client
            .head(download_url)
            .header("referer", &game_info.android_links[0]);

        let mut completed_length = Size::default();
        let length = Size::from_byte(loop {
            if let Ok(res) = request_head.try_clone().unwrap().send().await {
                break res.content_length().unwrap() as u64;
            }
        });
        if file_path.exists() {
            completed_length.set_byte(file_path.metadata().unwrap().len() as u64);
        }
        let mut file_handle = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .write(true)
            .open(&file_path)
            .await
            .unwrap();
        'restart: loop {
            let mut download_handle = loop {
                match request
                    .try_clone()
                    .unwrap()
                    .header(
                        "range",
                        format!("bytes={}-", completed_length.as_byte()),
                    )
                    .send()
                    .await
                {
                    Ok(handle) => {
                        break handle;
                    }
                    Err(err) => {
                        Console::clear_line();
                        println!("{:?}", err);
                        if err.is_timeout() || err.is_connect() || err.is_request() {
                            continue 'restart;
                        } else {
                            break 'restart;
                        }
                    }
                }
            };
            if completed_length == length {
                break;
            }
            loop {
                if let Ok(chunk) = download_handle.chunk().await {
                    if let Some(chunk) = chunk {
                        completed_length += Size::from_byte(chunk.len() as u64);
                        file_handle.write(&chunk).await.unwrap();
                    }
                }
                Console::clear_line();
                print!(
                    "{}",
                    Console::format_download_game(completed_length, length, &file_path_str)
                );
                stdout().flush().unwrap();
                if completed_length == length {
                    break;
                }
            }
            break;
        }
        Console::clear_line();
        Ok(())
    }

    pub async fn comic_download_eps(&self, cid: &str, savedir: &str) -> Result<(), Error> {
        let mut page_index = 1;
        loop {
            let eps = self.comic_eps(cid, page_index).await?;
            for ep in eps.iter() {
                self.comic_download_ep(cid, ep.order.unwrap(), savedir)
                    .await?;
            }
            if !eps.has_next() {
                break;
            }
            page_index = eps.next();
        }
        Ok(())
    }

    pub async fn comic_download_ep(
        &self,
        cid: &str,
        index: u64,
        savedir: &str,
    ) -> Result<(), Error> {
        let mut page_index = 1;
        let mut _comics_completed_total = Arc::new(RwLock::new(0));
        loop {
            let pages = self.comic_pages(cid, index, page_index).await?;
            let metadata = self.comic_metadata(cid).await?;
            let output_dir = PathBuf::from_str(savedir).unwrap().join(&format!(
                "{} - {}",
                &metadata.metadata.title, &metadata.metadata.author
            ));
            if !output_dir.exists() {
                fs::create_dir_all(&output_dir).await?;
            }
            let sub_savepath = output_dir.join(pages.ep.title.as_str());
            if !sub_savepath.exists() {
                fs::create_dir_all(&sub_savepath).await?;
            }
            let downloading_name = format!(
                "{} of {} - {}",
                pages.ep.title.as_str(),
                &metadata.metadata.title,
                &metadata.metadata.author
            );
            let mut _comics_total_length = Arc::new(RwLock::new(0));
            let mut _comics_completed_length = Arc::new(RwLock::new(0));
            let mut _comics_downloaded = Arc::new(RwLock::new(0));
            for comic in pages.iter() {
                let comic = comic;
                let comics_downloaded = _comics_downloaded.clone();
                let comics_completed_length = _comics_completed_length.clone();
                let comics_total_length = _comics_total_length.clone();
                let comics_completed_total = _comics_completed_total.clone();
                let file_path = sub_savepath.join(path_escape(comic.media.filename()));
                let download_url = comic.media.download_url();
                let request = self.get(download_url.as_str());
                let request_head = self.client.head(download_url.as_str());
                tokio::spawn(async move {
                    let length = loop {
                        if let Ok(res) = request_head.try_clone().unwrap().send().await {
                            break res.content_length().unwrap();
                        }
                    };
                    *comics_total_length.write().await += length;
                    let mut completed_length = 0;
                    if file_path.exists() {
                        completed_length = file_path.metadata().unwrap().len();
                        *comics_completed_length.write().await += completed_length;
                    }
                    if completed_length == length {
                        *comics_downloaded.write().await += 1;
                        *comics_completed_total.write().await += 1;
                        return;
                    }
                    'restart: loop {
                        let mut download_handle = loop {
                            match request
                                .try_clone()
                                .unwrap()
                                .header(
                                    "range",
                                    format!("bytes={}-", completed_length),
                                )
                                .send()
                                .await
                            {
                                Ok(handle) => {
                                    break handle;
                                }
                                Err(err) => {
                                    Console::clear_line();
                                    println!("{:?}", err);
                                    if err.is_timeout() || err.is_connect() || err.is_request() {
                                        continue 'restart;
                                    } else {
                                        break 'restart;
                                    }
                                }
                            }
                        };
                        let mut file_handle = fs::OpenOptions::new()
                            .create(true)
                            .write(true)
                            .append(true)
                            .open(&file_path)
                            .await
                            .unwrap();
                        while completed_length < length {
                            if let Ok(chunk) = download_handle.chunk().await {
                                if let Some(chunk) = chunk {
                                    completed_length += chunk.len() as u64;
                                    *comics_completed_length.write().await += chunk.len() as u64;
                                    file_handle.write(&chunk).await.unwrap();
                                }
                            } else {
                                continue 'restart;
                            }
                        }
                        break;
                    }
                    *comics_downloaded.write().await += 1;
                    *comics_completed_total.write().await += 1;
                });
            }
            while *_comics_downloaded.read().await < pages.len() {
                Console::clear_line();
                print!(
                    "{}",
                    Console::format_download_ep(
                        &downloading_name,
                        pages.current(),
                        pages.pages,
                        *_comics_downloaded.read().await as u64 + 1,
                        pages.len() as u64,
                        *_comics_completed_total.read().await + 1,
                        pages.total
                    )
                );
                stdout().flush().unwrap();
                tokio::time::sleep(Duration::from_millis(50)).await;
            }
            Console::clear_line();
            if !pages.has_next() {
                break;
            }
            page_index = pages.next();
        }
        Ok(())
    }
}
