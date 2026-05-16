use httpfromscratch::response::HttpResponse;
use httpfromscratch::request::Method;

#[test]
fn ok_to_bytes() {
    let res = HttpResponse::ok(b"hello".to_vec(), "text/plain");
    let raw_res = res.to_bytes();
    let str_res = String::from_utf8(raw_res).unwrap();

    assert!(str_res.starts_with("HTTP/1.1 200 OK\r\n"));
    assert!(str_res.contains("Content-Length: 5\r\n"));
    assert!(str_res.contains("Content-Type: text/plain"));
    assert!(str_res.ends_with("\r\n\r\nhello"));
}

#[test]
fn not_found_res() {
    let res = HttpResponse::not_found();
    let raw_res = res.to_bytes();
    let str_res = String::from_utf8(raw_res).unwrap();

    assert!(str_res.starts_with("HTTP/1.1 404"));
    assert_eq!(res.status, 404);
}

#[test]
fn bad_req_res(){
    let res = HttpResponse::bad_request("idk bruh");
    let raw_res = res.to_bytes();
    let str_res = String::from_utf8(raw_res).unwrap();

    assert_eq!(res.status, 400);
    assert!(str_res.contains("400"));
    assert!(str_res.contains("idk bruh"));
}

#[test]
fn meth_not_allowed_res(){
    let res = HttpResponse::method_not_allowed(&[Method::GET, Method::HEAD]);
    let raw_res = res.to_bytes();
    let str_res = String::from_utf8(raw_res).unwrap();

    assert!(str_res.contains("405"));
    assert_eq!(res.status, 405);
    assert!(str_res.contains("Allow"));
    assert!(str_res.contains("GET"));
    assert!(str_res.contains("HEAD"));
}

#[test]
fn internal_res(){
    let res = HttpResponse::internal_error();
    assert_eq!(res.status, 500);
    assert_eq!(res.reason, String::from("something went wrong..."));
}

