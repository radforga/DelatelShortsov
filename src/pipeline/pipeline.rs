
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
        pub async  fn run(&self, job: Job){
            match job.source{
                VideoSource::Url(url) => {
                        self.downloader.download_vid(url).await;
                    }

                VideoSource::File(path) => {
                    print!("Тут должен быть юрл до видоса");
                }
            }
            
        }

}
