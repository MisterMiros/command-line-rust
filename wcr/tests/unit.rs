use wcr::FileInfo;
use std::io::Cursor;

#[test]
fn test_count() {
    let text = "I don't want the world. I just want your half.\r\n";
    let info = wcr::count(&Some("filename"), Cursor::new(text));
    assert!(info.is_ok());
    let expected = FileInfo {
        name: Some(String::from("filename")),
        num_lines: 1,
        num_words: 10,
        num_chars: 48,
        num_bytes: 48,
    };
    assert_eq!(info.unwrap(), expected);
}
