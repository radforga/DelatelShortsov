use std::{any::TypeId, path::PathBuf};

use ffmpeg_next;



pub struct Decoder {
    input: ffmpeg_next::format::context::Input,
    video_decoder: ffmpeg_next::codec::decoder::Video,
    video_stream_index: usize,
    packet: ffmpeg_next::Packet,
    frame: ffmpeg_next::frame::Video,
}

impl Decoder{
    pub fn new(path : PathBuf)->Self{
     ffmpeg_next::init().unwrap();
        let mut input = ffmpeg_next::format::input(&path).unwrap();
        let (video_stream_index, video_decoder) ={
        let video_stream = input.streams().best(ffmpeg_next::media::Type::Video).unwrap();
        let context = ffmpeg_next::codec::context::Context::from_parameters(video_stream.parameters()).unwrap();
                let video_stream_index = video_stream.index();
                let video_decoder = context.decoder().video().unwrap();
                (video_stream_index, video_decoder)
        };
        Self { input, video_decoder, video_stream_index, packet: ffmpeg_next::Packet::empty(), frame: ffmpeg_next::frame::Video::empty() }
    }
    pub fn next_frame(&mut self) -> Option<&ffmpeg_next::frame::Video>{
           loop {
            if self.packet.read(&mut self.input).is_err(){
                return  None;
            }
            if self.packet.stream() != self.video_stream_index{
                continue;
            }

            self.video_decoder.send_packet(&self.packet).unwrap();

            if self.video_decoder.receive_frame(&mut self.frame).is_ok(){
                return Some(&self.frame);
            }

        }

    }
}
