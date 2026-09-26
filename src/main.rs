use std::io;
use std::path::PathBuf;
use std::time::Duration;

mod downloader;
mod pipeline;
mod editer;





use crate::pipeline::Job;
use crate::downloader::YoutubeDownLoader;
use crate::pipeline::Pipeline;
use crate::editer::Decoder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pipeline = Pipeline::new();
    let mut job = Job::new();
    println!("Введите ссылку на видео (ютуб)");

    let mut url = String::new();
    io::stdin().read_line(&mut url).expect("Не удалосб прочитать строку :(");
    job.edit_source_url(url);
    
    let (start,end) = Pipeline::read_clip_range();

    println!("Запуск скачивания...");
    
    pipeline.run(&mut job).await;
    
    println!("Скачивание успешно завершено.");
    println!("Декодирование");

    let mut decoder = match &job.source {
                              pipeline::job::VideoSource::File(file) => {
                                  Decoder::new(file.clone()) }

                            pipeline::job::VideoSource::Url(url) =>{
                                Decoder::new(PathBuf::from(url))
                            }
                      };
    let time_base = decoder.time_base;
    while let Some(frame)= decoder.next_frame() {
        if frame.pts().unwrap() as f64 * f64::from(time_base) >= start.as_secs_f64()&& frame.pts().unwrap() as f64 * f64::from(time_base) <=end.as_secs_f64(){
           println!(
                "frame: {}x{}, time_base: {}",
              frame.width(),
                frame.height(),
                frame.pts().unwrap() as f64 * f64::from(time_base)
                );
               }
    }
        Ok(())
}
