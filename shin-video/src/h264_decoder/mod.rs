mod av;
mod y4m;

use std::io::{Read, Seek};
use std::iter::Peekable;
use std::sync::Arc;
use std::thread;
pub use y4m::{BitsPerSample, Colorspace, Frame, FrameSize, PlaneSize};

use crate::mp4::Mp4TrackReader;
use crate::mp4_bitstream_converter::Mp4BitstreamConverter;
use anyhow::{bail, Result};
use tracing::{debug, error, trace};

use self::av::AVH264Decoder;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FrameTiming {
    pub frame_number: u32,
    pub start_time: u64,
    pub duration: u32,
}

/// Decodes h264 Annex B format to YUV420P (or, possibly, some other frame format).
///
/// Currently implemented as a pipe to the ffmpeg binary, other options are possible in the future.
pub struct H264Decoder {
    #[allow(unused)]
    ffmpeg_decoder: Arc<AVH264Decoder>,

    frame_receiver: Peekable<std::sync::mpsc::IntoIter<Frame>>,
    frame_timing_receiver: std::sync::mpsc::Receiver<FrameTiming>,

    #[allow(unused)]
    frame_size: Option<FrameSize>,
}

impl H264Decoder {
    pub fn new<S: Read + Seek + Send + 'static>(track: Mp4TrackReader<S>) -> Result<Self> {
        let ffmpeg_decoder = Arc::new(AVH264Decoder::new().unwrap());

        trace!("test");

        // send the decoded frames from ffmpeg to the game
        let (frame_sender, frame_receiver) = std::sync::mpsc::sync_channel(1);
        // send the frame timings from the mp4 stream to the game (without passing through ffmpeg)
        // this has a bit more delay than other chans because it goes around ffmpeg and ffmpeg has its own delay of several frames
        // hence the larger capacity (otherwise we might deadlock)
        let (frame_timing_sender, frame_timing_receiver) = std::sync::mpsc::sync_channel(10);

        {
            let ffmpeg_decoder = Arc::clone(&ffmpeg_decoder);
            thread::spawn(move || loop {
                match ffmpeg_decoder.as_ref().receive() {
                    Ok(Some(frame)) => {
                        trace!("Sending frame to game");
                        if frame_sender.send(frame).is_err() {
                            debug!("Game closed the channel, stopping sending frames");
                            break;
                        }
                    }
                    Ok(None) => {
                        trace!("No frame available at the moment");
                    },
                    Err(e) => {
                        error!("Error reading frame from ffmpeg: {:?}", e);
                        break;
                    }
                }
            });
        }

        {
            let ffmpeg_decoder = Arc::clone(&ffmpeg_decoder);
            thread::spawn(move || {
                let mut track = track;
                let mut buffer = Vec::new();

                let mut bitstream_converter: Mp4BitstreamConverter =
                    track.get_mp4_track_info(Mp4BitstreamConverter::for_mp4_track);

                let mut frame_number = 0;

                loop {
                    match track.next_sample() {
                        Ok(Some(sample)) => {
                            // MP4 can do frame reordering if B-frames are used
                            // this seems to be indicated by the sample.rendering_offset field
                            // it also seems that this info can be duplicated in the h264 bistream in a form of picture_timing SEI NALUs
                            // it also seems that ffmpeg handles the picture_timing SEI NALUs correctly, so we don't need to do anything with the rendering offset if they are present
                            // NOTE: maybe what I said above is not correct, as, after stripping the SEI NALUs, the video still looks correct...
                            // I'll ignore the rendering offset for now, but this should be accounted for if other decoders are implemented

                            let frame_timing = FrameTiming {
                                frame_number,
                                start_time: sample.start_time,
                                duration: sample.duration,
                            };

                            frame_number += 1;

                            if frame_timing_sender.send(frame_timing).is_err() {
                                debug!("Game closed the channel, stopping sending frame timings");
                                break;
                            }

                            bitstream_converter.convert_packet(&sample.bytes, &mut buffer);
                            trace!("Sending sample to ffmpeg ({} bytes)", buffer.len());
                            match ffmpeg_decoder.decode(buffer.as_slice()) {
                                Ok(_) => {}
                                Err(e) => {
                                    error!("Error writing sample to ffmpeg: {:?}", e);
                                    break;
                                }
                            }
                        }
                        Ok(None) => {
                            debug!("EOF from mp4, stopping sending to ffmpeg");
                            break;
                        }
                        Err(e) => {
                            error!("Error reading sample from mp4: {}", e);
                            break;
                        }
                    }
                }
            });
        }

        let frame_receiver = frame_receiver.into_iter().peekable();

        Ok(Self {
            ffmpeg_decoder,
            frame_receiver,
            frame_timing_receiver,
            frame_size: None,
        })
    }

    pub fn read_frame(&mut self) -> Result<Option<(FrameTiming, Frame)>> {
        trace!("Reading frame from ffmpeg...");
        match self.frame_receiver.next() {
            Some(frame) => {
                let timing = self.frame_timing_receiver.recv().unwrap();

                self.frame_size = Some(*frame.size());
                Ok(Some((timing, frame)))
            }
            None => Ok(None),
        }
    }

    pub fn frame_size(&mut self) -> Result<FrameSize> {
        debug!("Reading frame info from ffmpeg");
        match self.frame_size {
            Some(info) => Ok(info),
            None => match self.frame_receiver.peek() {
                Some(frame) => {
                    self.frame_size = Some(*frame.size());
                    Ok(*frame.size())
                }
                None => {
                    bail!("No frames available, don't know the format")
                }
            },
        }
    }
}
