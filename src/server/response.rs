//! # Server Responses
//!
//! These are responses sent by a `hyper::Server` to clients, after
//! receiving a request.
use std::io::IoResult;
use std::io::net::tcp::TcpStream;

use header;
use status;
use version;
use rfc7230::{CR, LF, LINE_ENDING};


/// The outgoing half for a Tcp connection, created by a `Server` and given to a `Handler`.
pub struct Response {
    /// The status code for the request.
    pub status: status::StatusCode,
    /// The outgoing headers on this response.
    pub headers: header::Headers,
    /// The HTTP version of this response.
    pub version: version::HttpVersion,

    headers_written: bool, // TODO: can this check be moved to compile time?
    body: TcpStream
}

impl Response {

    /// Creates a new Response that can be used to write to a network stream.
    pub fn new(tcp: TcpStream) -> Response {
        Response {
            status: status::Ok,
            version: version::Http11,
            headers: header::Headers::new(),
            headers_written: false,
            body: tcp
        }
    }

    fn write_head(&mut self) -> IoResult<()> {
        if self.headers_written {
            debug!("headers previsouly written, nooping");
            return Ok(());
        }
        self.headers_written = true;
        debug!("writing head: {} {}", self.version, self.status);
        try!(write!(self.body, "{} {}{}{}", self.version, self.status, CR as char, LF as char));

        for (name, header) in self.headers.iter() {
            debug!("headers {}: {}", name, header);
            try!(write!(self.body, "{}: {}", name, header));
            try!(self.body.write(LINE_ENDING));
        }

        self.body.write(LINE_ENDING)
    }

    /// Flushes all writing of a response to the client.
    pub fn end(&mut self) -> IoResult<()> {
        try!(self.flush());
        self.body.close_write()
    }
}


impl Writer for Response {
    fn write(&mut self, msg: &[u8]) -> IoResult<()> {
        if !self.headers_written {
            try!(self.write_head());
        }
        debug!("write {:u} bytes", msg.len());
        self.body.write(msg)
    }

    fn flush(&mut self) -> IoResult<()> {
        if !self.headers_written {
            try!(self.write_head());
        }
        self.body.flush()
    }
}