use clap::{ArgAction, Parser};


#[derive(Parser, Debug, Clone)]
pub struct GlobalOptions {
    #[clap(short = 'a', long = "all-proxy")]
    pub all_proxy: Option<String>,
    #[clap(long = "http-proxy")]
    pub http_proxy: Option<String>,
    #[clap(long = "https-proxy")]
    pub https_proxy: Option<String>,
    #[clap(subcommand)]
    pub subcommand: SubCommand,
    #[clap(short = 'o', long = "save-dir", default_value = ".")]
    pub save_dir: String,
    #[clap(short='d', long="download", default_value="false", action=ArgAction::SetTrue)]
    pub download: bool,
}

#[derive(Parser, Debug, Clone)]
pub enum SubCommand {
    #[clap(subcommand)]
    Comic(ComicOptions),
    #[clap(subcommand)]
    Game(GameOptions),
    #[clap(subcommand)]
    User(UserOptions),
}

#[derive(Parser, Debug, Clone)]
pub enum ComicOptions {
    Ranking,
    Metadata {
        #[clap(short='c', long="cids", action=ArgAction::Append)]
        cids: Vec<String>,
        #[clap(short = 'o', long = "save-dir")]
        save_dir: Option<String>,
    },
    Recommended {
        #[clap(short='c', long="cids", action=ArgAction::Append)]
        cids: Vec<String>,
        #[clap(short = 'o', long = "save-dir")]
        save_dir: Option<String>,
    },
    Eps {
        #[clap(short = 'c', long = "cid")]
        cid: String,
        #[clap(short = 's', long = "start", default_value = "1")]
        start: u64,
        #[clap(short = 'u', long = "until")]
        end: Option<u64>,
        #[clap(short = 'o', long = "save-dir")]
        save_dir: Option<String>,
    },
    Pages {
        #[clap(short = 'c', long = "cid")]
        cid: String,
        #[clap(long = "start-index", default_value = "1")]
        start_index: u64,
        #[clap(long = "until-index")]
        end_index: Option<u64>,
        #[clap(short = 's', long = "start", default_value = "1")]
        start: u64,
        #[clap(short = 'u', long = "until")]
        end: Option<u64>,
        #[clap(short = 'o', long = "save-dir")]
        save_dir: Option<String>,
    },
    PicLikeGet {
        #[clap(short = 'c', long = "cid")]
        cid: String,
        #[clap(short = 's', long = "start", default_value = "1")]
        start: u64,
        #[clap(short = 'u', long = "until")]
        end: Option<u64>,
        #[clap(short = 'o', long = "save-dir")]
        save_dir: Option<String>,
    },
    Search {
        #[clap(short = 'k', long = "keyword")]
        keyword: String,
        #[clap(short = 's', long = "start", default_value = "1")]
        start: u64,
        #[clap(short = 'u', long = "until")]
        end: Option<u64>,
        #[clap(short = 'o', long = "save-dir")]
        save_dir: Option<String>,
    },
    Favourites {
        #[clap(short = 's', long = "start", default_value = "1")]
        start: u64,
        #[clap(short = 'u', long = "until")]
        end: Option<u64>,
        #[clap(short = 'o', long = "save-dir")]
        save_dir: Option<String>,
    },
    Download {
        #[clap(short = 'c', long = "cid")]
        cids: Vec<String>,
        #[clap(short = 'o', long = "save-dir")]
        save_dir: Option<String>,
    },
}

#[derive(Parser, Debug, Clone)]
pub enum GameOptions {
    Games {
        #[clap(short = 's', long = "start", default_value = "1")]
        start: u64,
        #[clap(short = 'u', long = "until")]
        end: Option<u64>,
        #[clap(short = 'o', long = "save-dir")]
        save_dir: Option<String>,
    },
    Info {
        #[clap(short='c', long="cids", action=ArgAction::Append)]
        cids: Vec<String>,
        #[clap(short = 'o', long = "save-dir")]
        save_dir: Option<String>,
    },
    Download {
        #[clap(short='c', long="cids", action=ArgAction::Append)]
        cids: Vec<String>,
        #[clap(short = 'o', long = "save-dir")]
        save_dir: Option<String>,
    },
}

#[derive(Parser, Debug, Clone)]
pub enum UserOptions {
    PunchIn,
    Profile,
}

pub enum DownloadParmas {
    Comic(),
    Game(),
}
