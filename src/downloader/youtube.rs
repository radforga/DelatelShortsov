use yt_dlp::Downloader;
use std::path::PathBuf;
use yt_dlp::client::deps::Libraries;

use crate::Job;

pub struct YoutubeDownLoader{
    
}

impl YoutubeDownLoader{
    pub fn new() -> Self {
        Self {  }
    }


    pub async fn download_vid(&self, url:String) -> Result<(), Box<dyn std::error::Error>>{
           let libraries_dir = PathBuf::from("libs");
    let output_dir = PathBuf::from("output");
    
    let youtube = libraries_dir.join("yt-dlp");
    let ffmpeg = libraries_dir.join("ffmpeg");
    
    let libraries = Libraries::new(youtube, ffmpeg);
    let downloader = Downloader::builder(libraries, output_dir)
        .build()
        .await?;

        
        let video = downloader.fetch_video_infos(url).await?;
        let video_path = downloader.download_video(&video, "test-vid.mp4").await?;
        Ok(())
    }
}
