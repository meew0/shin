use std::mem::size_of;
use std::ptr::null_mut;
use std::slice::from_raw_parts;

use ffmpeg_sys_next::*;

use super::{BitsPerSample, Frame, FrameSize, PlaneSize};

#[derive(Debug)]
pub enum InitError {
    H264CodecNotFound,
    ParserInitFailed,
    CodecAllocFailed,
    FrameAllocFailed,
    PacketAllocFailed,
    CodecOpenFailed(i32),
}

#[derive(Debug)]
pub enum DecodeError {
    ParseFailed(i32),
    PacketSendFailed(i32),
}

#[derive(Debug)]
pub enum ReceiveError {
    ReceiveFailed(i32),
    EndOfFile,
}

pub struct AVH264Decoder {
    frame: *mut AVFrame,
    packet: *mut AVPacket,
    dec_ctx: *mut AVCodecContext,
    parser: *mut AVCodecParserContext,
}

unsafe impl Send for AVH264Decoder {}
unsafe impl Sync for AVH264Decoder {}

unsafe extern "C" fn select_pixel_format(
    _context: *mut AVCodecContext,
    fmt: *const AVPixelFormat,
) -> AVPixelFormat {
    let size = size_of::<AVPixelFormat>();
    let mut i = 0;
    loop {
        match *(fmt.add(i * size)) {
            // If we have YUV420P available, select it.
            AVPixelFormat::AV_PIX_FMT_YUV420P => return AVPixelFormat::AV_PIX_FMT_YUV420P,

            // NONE signals the end of the list of available pixel formats. It seems that YUV420P is not available.
            AVPixelFormat::AV_PIX_FMT_NONE => return AVPixelFormat::AV_PIX_FMT_NONE,
            _ => {}
        }
        i += 1;
    }
}

unsafe fn copy_plane(data: *const u8, linesize: i32, width: i32, height: i32) -> Vec<u8> {
    let mut result = Vec::with_capacity((width * height).try_into().unwrap());

    for y in 0..height {
        let slice = from_raw_parts(
            data.add((y * linesize).try_into().unwrap()),
            width.try_into().unwrap(),
        );
        result.extend_from_slice(slice);
    }

    result
}

fn convert_frame(av_frame: *const AVFrame) -> Frame {
    unsafe {
        let width = (*av_frame).width;
        let height = (*av_frame).height;

        let y_plane = copy_plane((*av_frame).data[0], (*av_frame).linesize[0], width, height);
        let u_plane = copy_plane((*av_frame).data[1], (*av_frame).linesize[1], width, height);
        let v_plane = copy_plane((*av_frame).data[2], (*av_frame).linesize[2], width, height);

        let plane_size = PlaneSize {
            width: width.try_into().unwrap(),
            height: height.try_into().unwrap(),
            bits_per_sample: BitsPerSample::B8,
        };

        Frame::new(
            [y_plane, u_plane, v_plane],
            None,
            FrameSize {
                colorspace: super::Colorspace::C420,
                plane_sizes: [plane_size, plane_size, plane_size],
            },
        )
    }
}

impl AVH264Decoder {
    pub fn new() -> Result<Self, InitError> {
        unsafe {
            let h264_codec = avcodec_find_decoder(AVCodecID::AV_CODEC_ID_H264);
            if h264_codec.is_null() {
                return Err(InitError::H264CodecNotFound);
            }

            let parser = av_parser_init((*h264_codec).id as i32);
            if parser.is_null() {
                return Err(InitError::ParserInitFailed);
            }

            let dec_ctx = avcodec_alloc_context3(h264_codec);
            if dec_ctx.is_null() {
                return Err(InitError::CodecAllocFailed);
            }

            // Set the function to select the desired pixel format
            (*dec_ctx).get_format = Some(select_pixel_format);

            let error_code = avcodec_open2(dec_ctx, h264_codec, null_mut());
            if error_code < 0 {
                return Err(InitError::CodecOpenFailed(error_code));
            }

            let frame = av_frame_alloc();
            if frame.is_null() {
                return Err(InitError::FrameAllocFailed);
            }

            let packet = av_packet_alloc();
            if frame.is_null() {
                return Err(InitError::PacketAllocFailed);
            }

            Ok(Self {
                frame,
                packet,
                dec_ctx,
                parser,
            })
        }
    }

    pub fn decode(&self, data: &[u8]) -> Result<(), DecodeError> {
        let mut pos = 0;
        let mut size = data.len();

        unsafe {
            while size > 0 {
                let parse_ret = av_parser_parse2(
                    self.parser,
                    self.dec_ctx,
                    &mut (*self.packet).data,
                    &mut (*self.packet).size,
                    data.as_ptr().add(pos),
                    data.len().try_into().unwrap(),
                    AV_NOPTS_VALUE,
                    AV_NOPTS_VALUE,
                    0,
                );

                if parse_ret < 0 {
                    return Err(DecodeError::ParseFailed(parse_ret));
                }

                pos += parse_ret as usize;
                size -= parse_ret as usize;

                if (*self.packet).size > 0 {
                    let send_ret = avcodec_send_packet(self.dec_ctx, self.packet);
                    if send_ret < 0 {
                        return Err(DecodeError::PacketSendFailed(send_ret));
                    }
                }
            }
        }

        Ok(())
    }

    pub fn receive(&self) -> Result<Option<Frame>, ReceiveError> {
        unsafe {
            let ret = avcodec_receive_frame(self.dec_ctx, self.frame);

            if ret == AVERROR(EAGAIN) {
                return Ok(None);
            }

            if ret == AVERROR_EOF {
                return Err(ReceiveError::EndOfFile);
            }

            if ret < 0 {
                return Err(ReceiveError::ReceiveFailed(ret));
            }

            Ok(Some(convert_frame(self.frame)))
        }
    }
}

impl Drop for AVH264Decoder {
    fn drop(&mut self) {
        unsafe {
            av_parser_close(self.parser);
            avcodec_free_context(&mut self.dec_ctx);
            av_frame_free(&mut self.frame);
            av_packet_free(&mut self.packet);
        }
    }
}
