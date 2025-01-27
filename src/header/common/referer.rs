header! {
    /// `Referer` header, defined in
    /// [RFC7231](http://tools.ietf.org/html/rfc7231#section-5.5.2)
    ///
    /// The `Referer` header field allows the user agent to specify a
    /// URI reference for the resource from which the target URI was obtained
    /// (i.e., the "referrer", though the field name is misspelled).  A user
    /// agent MUST NOT include the fragment and userinfo components of the
    /// URI reference, if any, when generating the Referer field value.
    ///
    /// # ABNF
    ///
    /// ```text
    /// Referer = absolute-URI / partial-URI
    /// ```
    ///
    /// # Example values
    ///
    /// * `http://www.example.org/hypertext/Overview.html`
    ///
    /// # Examples
    ///
    /// ```
    /// use hyperx::header::{Headers, Referer};
    ///
    /// let mut headers = Headers::new();
    /// headers.set(Referer::new("/People.html#tim"));
    /// ```
    ///
    /// ```
    /// use hyperx::header::{Headers, Referer};
    ///
    /// let mut headers = Headers::new();
    /// headers.set(Referer::new("http://www.example.com/index.html"));
    /// ```
    // TODO Use URL
    (Referer, "Referer") => Cow[str]

    test_referer {
        // Testcase from the RFC
        test_header!(test1, vec![b"http://www.example.org/hypertext/Overview.html"]);
    }
}

bench_header!(bench, Referer, { vec![b"http://foo.com/hello:3000".to_vec()] });

standard_header!(Referer, REFERER);
