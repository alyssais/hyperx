use header::HttpDate;

header! {
    /// `Expires` header, defined in [RFC7234](http://tools.ietf.org/html/rfc7234#section-5.3)
    ///
    /// The `Expires` header field gives the date/time after which the
    /// response is considered stale.
    ///
    /// The presence of an Expires field does not imply that the original
    /// resource will change or cease to exist at, before, or after that
    /// time.
    ///
    /// # ABNF
    ///
    /// ```text
    /// Expires = HTTP-date
    /// ```
    ///
    /// # Example values
    /// * `Thu, 01 Dec 1994 16:00:00 GMT`
    ///
    /// # Example
    ///
    /// ```
    /// use hyperx::header::{Headers, Expires};
    /// use std::time::{SystemTime, Duration};
    ///
    /// let mut headers = Headers::new();
    /// let expiration = SystemTime::now() + Duration::from_secs(60 * 60 * 24);
    /// headers.set(Expires(expiration.into()));
    /// ```
    (Expires, "Expires") => [HttpDate]

    test_expires {
        // Testcase from RFC
        test_header!(test1, vec![b"Thu, 01 Dec 1994 16:00:00 GMT"]);
    }
}

bench_header!(imf_fixdate, Expires, { vec![b"Sun, 07 Nov 1994 08:48:37 GMT".to_vec()] });
bench_header!(rfc_850, Expires, { vec![b"Sunday, 06-Nov-94 08:49:37 GMT".to_vec()] });
bench_header!(asctime, Expires, { vec![b"Sun Nov  6 08:49:37 1994".to_vec()] });

standard_header!(Expires, EXPIRES);
