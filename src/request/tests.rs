use crate::request::RequestLine;
use crate::HttpMethod;

#[test]
fn request_line_from_string_simple_string_expect_success() {
    let request_line_string = String::from("GET /foo/bar HTTP/1.1");

    let expected = RequestLine {
        method: HttpMethod::GET,
        target: Box::from("/foo/bar"),
        version: Box::from("HTTP/1.1"),
    };

    let result = RequestLine::from_string(&request_line_string).unwrap();

    assert_eq!(expected, result);
}
