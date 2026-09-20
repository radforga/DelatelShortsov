
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
        pub async  fn run(&self,mut job: Job){
 
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

        

}
