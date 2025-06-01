use std::io::{Cursor, Write};

use byteorder::WriteBytesExt;
use ddp::api::{BitsPerChannel, DDPHeader, DataType, Flags};
use ddp::sequence::Sequence;
use hue::stream::HueStreamLightsV2;
use tokio::net::UdpSocket;

use crate::error::ApiResult;

pub struct EntStream {
    seq: Sequence,
    socket: UdpSocket,
}

impl EntStream {
    #[allow(clippy::new_without_default)]
    #[must_use]
    pub const fn new(socket: UdpSocket) -> Self {
        Self {
            seq: Sequence::new(),
            socket,
        }
    }

    pub const fn start_stream(&mut self) -> ApiResult<()> {
        self.seq.reset();
        Ok(())
    }

    pub const fn stop_stream(&mut self) -> ApiResult<()> {
        self.seq.reset();
        Ok(())
    }

    pub async fn frame(&mut self, frame: &HueStreamLightsV2) -> ApiResult<()> {
        let length = 3 * frame.len();

        let mut buf = Cursor::new(Vec::with_capacity(DDPHeader::SIZE + length));

        let hdr = DDPHeader::new(Flags::PUSH, DataType::RGB, BitsPerChannel::Bits8)
            .with_length(u16::try_from(length)?)
            .with_sequence(self.seq.next());

        buf.write_all(&hdr.pack()?)?;

        match frame {
            HueStreamLightsV2::Rgb(rgb16_v2s) => {
                for rgb in rgb16_v2s {
                    buf.write_u8((rgb.rgb.r >> 8) as u8)?;
                    buf.write_u8((rgb.rgb.g >> 8) as u8)?;
                    buf.write_u8((rgb.rgb.b >> 8) as u8)?;
                }
            }
            HueStreamLightsV2::Xy(xy16_v2s) => {
                for xy in xy16_v2s {
                    let (xy, b) = xy.xy.to_xy();
                    let [r, g, b] = xy.to_rgb(b);
                    buf.write_u8(r)?;
                    buf.write_u8(g)?;
                    buf.write_u8(b)?;
                }
            }
        }

        self.socket.send(&buf.into_inner()).await?;

        Ok(())
    }
}
