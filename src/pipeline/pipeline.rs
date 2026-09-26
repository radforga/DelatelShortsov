use std::io;
use std::time::Duration;

use crate::downloader::YoutubeDownLoader;
use crate::Job;
use crate::pipeline::job::VideoSource;

pub struct Pipeline{
    downloader: YoutubeDownLoader,   
}

impl Pipeline{
        pub fn new() -> Self{
            Self { downloader: YoutubeDownLoader::new() }
        }
        pub async  fn run(&self,mut job: &mut Job){
 
            let video_path= match &job.source{
                VideoSource::Url(url) => {
                        self.downloader.download_vid(url.to_string()).await
                    }

                VideoSource::File(path) => {
                    Ok(path.clone())
                }
            };

            job.edit_source_path(video_path.expect("Чёт пошло не так при записи пути файла"));
            
        }
        
        
pub fn read_clip_range() -> (Duration, Duration) {
    loop {
        println!("Введите промежуток (MM:SS-MM:SS):");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Не удалось прочитать строку");

        let parts: Vec<&str> = input.trim().split('-').collect();

        if parts.len() != 2 {
            println!("Неверный формат. Пример: 12:30-15:45");
            continue;
        }

        let start = match Self::parse_timestamp(parts[0]) {
            Some(time) => time,
            None => {
                println!("Неверный таймкод начала.");
                continue;
            }
        };

        let end = match Self::parse_timestamp(parts[1]) {
            Some(time) => time,
            None => {
                println!("Неверный таймкод конца.");
                continue;
            }
        };

        if start >= end {
            println!("Начало должно быть раньше конца.");
            continue;
        }

        return (start, end);
    }
}

fn parse_timestamp(input: &str) -> Option<Duration> {
    let parts: Vec<u64> = input
        .trim()
        .split(':')
        .map(|part| part.parse().ok())
        .collect::<Option<Vec<_>>>()?;

    match parts.as_slice() {
        [minutes, seconds] if *seconds < 60 => {
            Some(Duration::from_secs(minutes * 60 + seconds))
        }

        [hours, minutes, seconds]
            if *minutes < 60 && *seconds < 60 =>
        {
            Some(Duration::from_secs(
                hours * 3600 + minutes * 60 + seconds
            ))
        }

        _ => None,
    }
}
}
        

        


