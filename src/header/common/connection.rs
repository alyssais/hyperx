use std::fmt::{self, Display};
use std::str::FromStr;
use unicase::Ascii;

pub use self::ConnectionOption::{KeepAlive, Close, ConnectionHeader};

static KEEP_ALIVE: &'static str = "keep-alive";
static CLOSE: &'static str = "close";

/// Values that can be in the `Connection` header.
#[derive(Clone, PartialEq, Debug)]
pub enum ConnectionOption {
    /// The `keep-alive` connection value.
    KeepAlive,
    /// The `close` connection value.
    Close,
    /// Values in the Connection header that are supposed to be names of other Headers.
    ///
    /// > When a header field aside from Connection is used to supply control
    /// > information for or about the current connection, the sender MUST list
    /// > the corresponding field-name within the Connection header field.
    // TODO: it would be nice if these "Strings" could be stronger types, since
    // they are supposed to relate to other Header fields (which we have strong
    // types for).
    ConnectionHeader(Ascii<String>),
}

impl FromStr for ConnectionOption {
    type Err = ();
    fn from_str(s: &str) -> Result<ConnectionOption, ()> {
        if Ascii::new(s) == KEEP_ALIVE {
            Ok(KeepAlive)
        } else if Ascii::new(s) == CLOSE {
            Ok(Close)
        } else {
            Ok(ConnectionHeader(Ascii::new(s.to_owned())))
        }
    }
}

impl Display for ConnectionOption {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            KeepAlive => "keep-alive",
            Close => "close",
            ConnectionHeader(ref s) => s.as_ref()
        })
    }
}

header! {
    /// `Connection` header, defined in
    /// [RFC7230](http://tools.ietf.org/html/rfc7230#section-6.1)
    ///
    /// The `Connection` header field allows the sender to indicate desired
    /// control options for the current connection.  In order to avoid
    /// confusing downstream recipients, a proxy or gateway MUST remove or
    /// replace any received connection options before forwarding the
    /// message.
    ///
    /// # ABNF
    ///
    /// ```text
    /// Connection        = 1#connection-option
    /// connection-option = token
    ///
    /// # Example values
    /// * `close`
    /// * `keep-alive`
    /// * `upgrade`
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// use hyperx::header::{Headers, Connection};
    ///
    /// let mut headers = Headers::new();
    /// headers.set(Connection::keep_alive());
    /// ```
    ///
    /// ```
    /// # extern crate hyperx;
    /// # extern crate unicase;
    /// # fn main() {
    /// // extern crate unicase;
    ///
    /// use hyperx::header::{Headers, Connection, ConnectionOption};
    /// use unicase::Ascii;
    ///
    /// let mut headers = Headers::new();
    /// headers.set(
    ///     Connection(vec![
    ///         ConnectionOption::ConnectionHeader(Ascii::new("upgrade".to_owned())),
    ///     ])
    /// );
    /// # }
    /// ```
    (Connection, "Connection") => (ConnectionOption)+

    test_connection {
        test_header!(test1, vec![b"close"]);
        test_header!(test2, vec![b"keep-alive"]);
        test_header!(test3, vec![b"upgrade"]);
    }
}

impl Connection {
    /// A constructor to easily create a `Connection: close` header.
    #[inline]
    pub fn close() -> Connection {
        Connection(vec![ConnectionOption::Close])
    }

    /// A constructor to easily create a `Connection: keep-alive` header.
    #[inline]
    pub fn keep_alive() -> Connection {
        Connection(vec![ConnectionOption::KeepAlive])
    }
}

bench_header!(close, Connection, { vec![b"close".to_vec()] });
bench_header!(keep_alive, Connection, { vec![b"keep-alive".to_vec()] });
bench_header!(header, Connection, { vec![b"authorization".to_vec()] });

#[cfg(test)]
mod tests {
    use super::{Connection,ConnectionHeader};
    use header::{Header, Raw};
    use unicase::Ascii;

    fn parse_option(header: Vec<u8>) -> Connection {
        let val: Raw = header.into();
        let connection: Connection = Header::parse_header(&val).unwrap();
        connection
    }

    #[test]
    fn test_parse() {
        assert_eq!(Connection::close(),parse_option(b"close".to_vec()));
        assert_eq!(Connection::keep_alive(),parse_option(b"keep-alive".to_vec()));
        assert_eq!(Connection::keep_alive(),parse_option(b"Keep-Alive".to_vec()));
        assert_eq!(Connection(vec![ConnectionHeader(Ascii::new("upgrade".to_owned()))]),
            parse_option(b"upgrade".to_vec()));
    }
}

standard_header!(Connection, CONNECTION);
