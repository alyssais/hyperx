use header::HttpDate;

header! {
    /// `Last-Modified` header, defined in
    /// [RFC7232](http://tools.ietf.org/html/rfc7232#section-2.2)
    ///
    /// The `Last-Modified` header field in a response provides a timestamp
    /// indicating the date and time at which the origin server believes the
    /// selected representation was last modified, as determined at the
    /// conclusion of handling the request.
    ///
    /// # ABNF
    ///
    /// ```text
    /// Expires = HTTP-date
    /// ```
    ///
    /// # Example values
    ///
    /// * `Sat, 29 Oct 1994 19:43:31 GMT`
    ///
    /// # Example
    ///
    /// ```
    /// use hyperx::header::{Headers, LastModified};
    /// use std::time::{SystemTime, Duration};
    ///
    /// let mut headers = Headers::new();
    /// let modified = SystemTime::now() - Duration::from_secs(60 * 60 * 24);
    /// headers.set(LastModified(modified.into()));
    /// ```
    (LastModified, "Last-Modified") => [HttpDate]

    test_last_modified {
        // Testcase from RFC
        test_header!(test1, vec![b"Sat, 29 Oct 1994 19:43:31 GMT"]);}
}

bench_header!(imf_fixdate, LastModified, { vec![b"Sun, 07 Nov 1994 08:48:37 GMT".to_vec()] });
bench_header!(rfc_850, LastModified, { vec![b"Sunday, 06-Nov-94 08:49:37 GMT".to_vec()] });
bench_header!(asctime, LastModified, { vec![b"Sun Nov  6 08:49:37 1994".to_vec()] });

standard_header!(LastModified, LAST_MODIFIED);
