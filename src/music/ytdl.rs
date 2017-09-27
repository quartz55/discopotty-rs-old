use std::process::{Command, Stdio};
use serde_json;

error_chain! {
    foreign_links {
        Io(::std::io::Error) #[cfg(unix)];
        Json(serde_json::error::Error);
    }
    errors {
    }
}

const BASE_ARGS: &'static [&'static str] = &[
    "--default-search", "ytsearch",
    "-f", "webm[abr>0]/bestaudio/best",
    "-i",
    "-j"
];

#[derive(Deserialize, Debug)]
pub struct YtdlTrackInfo {
    pub id: String,
    pub url: String,
    pub title: String,
    pub duration: u64
}

#[derive(Deserialize, Debug)]
pub struct YtdlFlatInfo {
    pub id: String
}

#[derive(Debug)]
pub enum YtdlResult {
    Track(YtdlTrackInfo),
    PlaylistInfo(YtdlFlatInfo)
}

pub fn parse_query_or_link(query_or_link: &str) -> Result<Vec<YtdlResult>> {
    let mut args = Vec::from(BASE_ARGS);
    let mut is_playlist = false;
    if query_or_link.contains("list=") {
        args.append(&mut vec!["--flat-playlist", query_or_link]);
        is_playlist = true;
    } else {
        args.append(&mut vec![query_or_link]);
    }

    let out = Command::new("youtube-dl")
        .args(&args)
        .stdin(Stdio::null())
        .output()?;

    if !out.status.success() {
        return Err("youtube-dl failed".into());
    }

    if is_playlist {
        let out_str = String::from_utf8(out.stdout).unwrap();
        let res = out_str.lines()
            .map(|info| {
                let flat_info: YtdlFlatInfo = serde_json::from_str(info).unwrap();
                YtdlResult::PlaylistInfo(flat_info)
            }).collect();
        return Ok(res);
    }

    let track_info: YtdlTrackInfo = serde_json::from_reader(&out.stdout[..])?;
    Ok(vec![YtdlResult::Track(track_info)])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_single_video() {
        let res = parse_query_or_link("https://www.youtube.com/watch?v=tgg15uNHCck").unwrap();
        for i in &res {
            println!("{:#?}", i);
        }
        assert_eq!(1, res.len());
        let song = res.first().unwrap();
        assert!(match song { &YtdlResult::Track(_) => true, _ => false});
    }

    #[test]
    fn parse_playlist() {
        let res = parse_query_or_link("https://www.youtube.com/playlist?list=PLhuLtJnYa2y4SyMqeA9xqbgzg79H0WSaz").unwrap();
        for i in &res {
            println!("{:#?}", i);
        }
        assert!(res.len() > 1);
        let info = res.first().unwrap();
        assert!(match info { &YtdlResult::PlaylistInfo(_) => true, _ => false});
    }

    #[test]
    fn parse_query_video() {
        let res = parse_query_or_link("lady gaga").unwrap();
        for i in &res {
            println!("{:#?}", i);
        }
        assert_eq!(1, res.len());
        let song = res.first().unwrap();
        assert!(match song { &YtdlResult::Track(_) => true, _ => false});
    }
}