#![allow(unused)]
use libpicacg::{
    error::Error,
    responses::{
        Comic, ComicMetadata, Creator, Ep, Game, GameInfo, Page, PictureDownloadResounce, Profile,
        PunchIn, RecommendPicLike, SearchRow,
    },
};
use size_utils::Size;

pub struct Console;

impl Console {
    pub fn clear_line() {
        print!("\r{}[K", 27 as char);
    }

    pub fn format_error(error: &Error) -> String {
        match error {
            Error::Api {
                code,
                message,
                error,
                detail,
            } => {
                format!(
                    "Error Code {} Message {} error {} detail: {}",
                    code, message, error, detail,
                )
            }
            _err => {
                format!("{:?}", error)
            }
        }
    }

    pub fn format_download_ep(
        name: &str,
        current_page: u64,
        total_pages: u64,
        page_image_completed: u64,
        page_image_total: u64,
        completed: u64,
        total: u64,
    ) -> String {
        format!(
            "{} Page {}/{} Completed Current Page Image {}/{} {:.02}% Total {}/{} {:.02}%",
            name,
            current_page,
            total_pages,
            page_image_completed,
            page_image_total,
            page_image_completed as f64 / page_image_total as f64 * 100.0,
            completed,
            total,
            completed as f64 / total as f64 * 100.0,
        )
    }

    pub fn format_punch_in(value: &PunchIn) -> String {
        format!(
            "Status {} LastPunchInDay {}",
            &value.status, &value.punch_in_last_day,
        )
    }

    pub fn format_creator(value: &Creator) -> String {
        format!(
            "Name[{}] Level[{}] Title[{}]",
            &value.name, value.level, &value.title,
        )
    }

    pub fn format_recommend_pic_like(value: &RecommendPicLike) -> String {
        format!(
            "Id[{}] Picture[{}] Title[{}]",
            &value.id, &value.pic, &value.title
        )
    }

    pub fn format_comic_metadata(value: &ComicMetadata) -> String {
        format!(
            "{} {}",
            Self::format_comic(&value.metadata),
            Self::format_creator(&value.creator)
        )
    }

    pub fn format_searchrow(value: &SearchRow) -> String {
        format!(
            "Id[{}] Author[{}] Likes[{}] Views[{}] Title[{}]",
            value.id, value.author, value.total_likes, value.total_views, value.title,
        )
    }

    pub fn format_ep(value: &Ep) -> String {
        format!("Id[{}] Title[{}]", value.id, value.title)
    }

    pub fn format_game(value: &Game) -> String {
        format!("Id[{}] Title[{}]", value.id, value.title)
    }

    pub fn format_game_info(value: &GameInfo) -> String {
        format!(
            "Id[{}] Size[{}MB] Likes[{}] Title[{}]",
            value.id, value.android_size, value.likes_count, value.title
        )
    }

    pub fn format_picture_download_resource(value: &PictureDownloadResounce) -> String {
        format!(
            "Filename[{}] DownloadUrl[{}]",
            value.filename(),
            value.download_url()
        )
    }

    pub fn format_download_game(completed: Size, length: Size, file_path: &str) -> String {
        format!(
            "Downloading {:.02}/{:.02}MB {:.02}% {}",
            completed.as_mb_f64(),
            length.as_mb_f64(),
            completed.as_kb_f64() / length.as_kb_f64() * 100.0,
            file_path
        )
    }

    pub fn format_page(value: &Page) -> String {
        format!(
            "Id[{}] {}",
            value.id,
            Self::format_picture_download_resource(&value.media)
        )
    }

    pub fn format_comic(value: &Comic) -> String {
        format!(
            "Id[{}] Author[{}] Likes[{}] Views[{}] Finished[{}] Title[{}]",
            &value.id,
            &value.author,
            &value.total_likes,
            &value.total_views,
            &value.finished,
            &value.title,
        )
    }

    pub fn format_profile(value: &Profile) -> String {
        format!(
            "Name[{}] Email[{}] Exp[{}] Level[{}] Punched[{}]",
            &value.name, &value.email, &value.exp, &value.level, &value.is_punched,
        )
    }
}
