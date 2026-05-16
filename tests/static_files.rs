use httpfromscratch::static_files::serve_file;
use std::fs::{write, remove_dir_all, create_dir_all};


fn create_test_dir() -> std::path::PathBuf{
    let dir = std::env::temp_dir().join("test_dir");
    let _ = remove_dir_all(&dir);
    create_dir_all(&dir).unwrap();
    return dir;
}
#[tokio::test]
async fn serve_html(){
    let dir = create_test_dir();
    write(dir.join("index.html"), "<h1>hi</h1>").unwrap();
    let res = serve_file(&dir, "/index.html").await.unwrap();
    assert_eq!(res.status, 200);
    assert_eq!(res.headers.get("Content-Type").unwrap(), "text/html");
    assert_eq!(res.body, b"<h1>hi</h1>");
}
//should probably try to serve other types too like json, js, etc.

#[tokio::test]
async fn no_double_dot() {
    let dir = create_test_dir();
    let res = serve_file(&dir, "/..").await;
    assert!(res.is_err());
}

#[tokio::test]
async fn no_null_byte() {
    let dir = create_test_dir();
    let res = serve_file(&dir, "/\0index.html").await;
    assert!(res.is_err());
}

