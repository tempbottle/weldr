use std::io;

use futures::{Poll, Async};
use tokio_proto::pipeline::Frame;

use bytes::{Buf, BufMut};
use bytes::ByteBuf;

use http;

use framed::{Parse, Serialize};

pub struct HttpParser {}

impl Parse for HttpParser {
    type Out = Frame<http::Response, http::Chunk, http::Error>;

    fn parse(&mut self, buf: &mut ByteBuf) -> Poll<Self::Out, io::Error> {
        if buf.len() == 0 {
            return Ok(
                Async::Ready(
                    Frame::Done
                )
            );
        }

        trace!("Attempting to parse bytes into HTTP Request");

        let response = http::Response(Vec::from(buf.bytes()));

        debug!("Parser created: {:?}", response);

        buf.clear();

        return Ok(
            Async::Ready(
                Frame::Message{
                    message: response,
                    body: false,
                }
            )
        );
    }
}

pub struct HttpSerializer {}

impl Serialize for HttpSerializer {

    type In = Frame<http::RequestHead, http::Chunk, http::Error>;

    /// Serializes a frame into the buffer provided.
    ///
    /// This method will serialize `msg` into the byte buffer provided by `buf`.
    /// The `buf` provided is an internal buffer of the `ProxyFramed` instance and
    /// will be written out when possible.
    fn serialize(&mut self, msg: Self::In, buf: &mut ByteBuf) {
        trace!("Serializing message frame: {:?}", msg);

        let input =
            b"GET / HTTP/1.1\r\n\
              Host: www.example.com\r\n\
              Accept: */*\r\n\
              \r\n";

        trace!("Trying to write {} bytes", input.len());
        buf.copy_from_slice(&input[..]);
        trace!("Copied {} bytes", input.len());
    }
}