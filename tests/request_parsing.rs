use httpfromscratch::request::{ self, Method};

#[test]
fn test_get_no_query(){
    let raw = b"GET /index.html HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n";
    let (req, consumed) = request::parse(raw).unwrap();
    assert_eq!(req.method, Method::GET);
    assert_eq!(req.path, "/index.html");
    assert_eq!(req.version, "HTTP/1.1");
    assert_eq!(consumed, raw.len());
    assert!(req.body.is_empty());
    assert_eq!(req.query, None);
}

#[test]
fn test_get_query() {
    let raw = b"GET /search?q=rust&page=1 HTTP/1.1\r\nHost: localhost\r\n\r\n";
    let (req, _) = request::parse(raw).unwrap();
    assert_eq!(req.path, "/search");
    assert_eq!(req.query, Some(String::from("q=rust&page=1")));

}

#[test]
fn test_post() {
    let raw = b"POST /api/echo HTTP/1.1\r\nHost: localhost\r\nContent-Length: 13\r\n\r\nHello, world!";
    let (req, consumed) = request::parse(raw).unwrap();
    assert_eq!(req.method, Method::POST);
    assert_eq!(req.path, "/api/echo");
    assert_eq!(req.body, b"Hello, world!");
    assert_eq!(consumed, raw.len());
}