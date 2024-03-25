use libpicacg::{Sort, Pagible};
use picacg::client::Client;
use reqwest::Proxy;
use std::io::{stdout, Write};


#[tokio::main]
async fn main() {
    let mut client = Client::new();
    let output_dir = "/media/disk_0/pika";
    client.set_proxy(Some(Proxy::all("http://localhost:15777").unwrap())).unwrap();
    client.login("yinpeach", "20050314yjc.").await.unwrap();
    let mut page_index = 1;
    loop {
        let favos = client.favorites(page_index, Sort::DescByDate).await.unwrap();
        for favo in favos.iter() {
            print!("\r{}[K", 27 as char);
            println!("Downloading {}", &favo.title);
            client.comic_download_eps(&favo.id, output_dir).await.unwrap();
            print!("\r{}[K", 27 as char);
            println!("Downloaded {}", &favo.title);
        }
        if !favos.has_next() {
            break;
        }
        page_index = favos.next();
    }
}
