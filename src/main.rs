use std::io;

mod downloader;
mod pipeline;

use crate::pipeline::Job;
use crate::downloader::YoutubeDownLoader;
use crate::pipeline::Pipeline;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pipeline = Pipeline::new();
    let mut job = Job::new();
    println!("Введите ссылку на видео (ютуб)");
    let mut url = String::new();
    io::stdin().read_line(&mut url).expect("Не удалосб прочитать строку :(");
    job.edit_source_url(url);
    println!("Запуск скачивания...");
    
    pipeline.run(job).await;
    
    println!("Скачивание успешно завершено.");
    Ok(())
}
