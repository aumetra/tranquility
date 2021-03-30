use {
    crate::{wrap_cow, wrap_cow_option, Request},
    http::{header::HeaderName, HeaderMap, HeaderValue},
};

const RSA_PUBLIC_KEY: &str = r#"
-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDCFENGw33yGihy92pDjZQhl0C3
6rPJj+CvfSC8+q28hxA161QFNUd13wuCTUcq0Qd2qsBe/2hFyc2DCJJg0h1L78+6
Z4UMR7EOcpfdUE9Hf3m/hs+FUR45uBJeDK1HSFHD8bHKD6kv8FPGfJTotc+2xjJw
oYi+1hqp1fIekaxsyQIDAQAB
-----END PUBLIC KEY-----
"#;

const RSA_PRIVATE_KEY: &str = r#"
-----BEGIN RSA PRIVATE KEY-----
MIICXgIBAAKBgQDCFENGw33yGihy92pDjZQhl0C36rPJj+CvfSC8+q28hxA161QF
NUd13wuCTUcq0Qd2qsBe/2hFyc2DCJJg0h1L78+6Z4UMR7EOcpfdUE9Hf3m/hs+F
UR45uBJeDK1HSFHD8bHKD6kv8FPGfJTotc+2xjJwoYi+1hqp1fIekaxsyQIDAQAB
AoGBAJR8ZkCUvx5kzv+utdl7T5MnordT1TvoXXJGXK7ZZ+UuvMNUCdN2QPc4sBiA
QWvLw1cSKt5DsKZ8UETpYPy8pPYnnDEz2dDYiaew9+xEpubyeW2oH4Zx71wqBtOK
kqwrXa/pzdpiucRRjk6vE6YY7EBBs/g7uanVpGibOVAEsqH1AkEA7DkjVH28WDUg
f1nqvfn2Kj6CT7nIcE3jGJsZZ7zlZmBmHFDONMLUrXR/Zm3pR5m0tCmBqa5RK95u
412jt1dPIwJBANJT3v8pnkth48bQo/fKel6uEYyboRtA5/uHuHkZ6FQF7OUkGogc
mSJluOdc5t6hI1VsLn0QZEjQZMEOWr+wKSMCQQCC4kXJEsHAve77oP6HtG/IiEn7
kpyUXRNvFsDE0czpJJBvL/aRFUJxuRK91jhjC68sA7NsKMGg5OXb5I5Jj36xAkEA
gIT7aFOYBFwGgQAQkWNKLvySgKbAZRTeLBacpHMuQdl1DfdntvAyqpAZ0lY0RKmW
G6aFKaqQfOXKCyWoUiVknQJAXrlgySFci/2ueKlIE1QqIiLSZ8V8OlpFLRnb1pzI
7U1yQXnTAEFYM560yJlzUpOb1V4cScGd365tiSMvxLOvTA==
-----END RSA PRIVATE KEY-----
"#;

const SIGNATURE_HEADER_VALUE: &str = r#"keyId="Test",algorithm="rsa-sha256",headers="date",signature="SjWJWbWN7i0wzBvtPl8rbASWz5xQW6mcJmn+ibttBqtifLN7Sazz6m79cNfwwb8DMJ5cou1s7uEGKKCs+FLEEaDV5lp7q25WqS+lavg7T8hc0GppauB6hbgEKTwblDHYGEtbGmtdHgVCk9SuS13F0hZ8FD0k/5OxEPXe5WozsbM=""#;
const AUTHORIZATION_HEADER_VALUE: &str = r#"Signature keyId="Test",algorithm="rsa-sha256",headers="date",signature="SjWJWbWN7i0wzBvtPl8rbASWz5xQW6mcJmn+ibttBqtifLN7Sazz6m79cNfwwb8DMJ5cou1s7uEGKKCs+FLEEaDV5lp7q25WqS+lavg7T8hc0GppauB6hbgEKTwblDHYGEtbGmtdHgVCk9SuS13F0hZ8FD0k/5OxEPXe5WozsbM=""#;

const BASIC_SIGNATURE_HEADER_VALUE: &str = r#"keyId="Test",algorithm="rsa-sha256",headers="(request-target) host date",signature="qdx+H7PHHDZgy4y/Ahn9Tny9V3GP6YgBPyUXMmoxWtLbHpUnXS2mg2+SbrQDMCJypxBLSPQR2aAjn7ndmw2iicw3HMbe8VfEdKFYRqzic+efkb3nndiv/x1xSHDJWeSWkx3ButlYSuBskLu6kd9Fswtemr3lgdDEmn04swr2Os0=""#;
const BASIC_AUTHORIZATION_HEADER_VALUE: &str = r#"Signature keyId="Test",algorithm="rsa-sha256",headers="(request-target) host date",signature="qdx+H7PHHDZgy4y/Ahn9Tny9V3GP6YgBPyUXMmoxWtLbHpUnXS2mg2+SbrQDMCJypxBLSPQR2aAjn7ndmw2iicw3HMbe8VfEdKFYRqzic+efkb3nndiv/x1xSHDJWeSWkx3ButlYSuBskLu6kd9Fswtemr3lgdDEmn04swr2Os0=""#;

const ALL_SIGNATURE_HEADER_VALUE: &str = r#"keyId="Test",algorithm="rsa-sha256",created=1402170695, expires=1402170699,headers="(request-target) (created) (expires) host date content-type digest content-length",signature="vSdrb+dS3EceC9bcwHSo4MlyKS59iFIrhgYkz8+oVLEEzmYZZvRs8rgOp+63LEM3v+MFHB32NfpB2bEKBIvB1q52LaEUHFv120V01IL+TAD48XaERZFukWgHoBTLMhYS2Gb51gWxpeIq8knRmPnYePbF5MOkR0Zkly4zKH7s1dE=""#;
const ALL_AUTHORIZATION_HEADER_VALUE: &str = r#"Signature keyId="Test",algorithm="rsa-sha256",created=1402170695, expires=1402170699,headers="(request-target) (created) (expires) host date content-type digest content-length",signature="vSdrb+dS3EceC9bcwHSo4MlyKS59iFIrhgYkz8+oVLEEzmYZZvRs8rgOp+63LEM3v+MFHB32NfpB2bEKBIvB1q52LaEUHFv120V01IL+TAD48XaERZFukWgHoBTLMhYS2Gb51gWxpeIq8knRmPnYePbF5MOkR0Zkly4zKH7s1dE=""#;

fn construct_headers() -> HeaderMap<HeaderValue> {
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("host"),
        HeaderValue::from_static("example.com"),
    );
    headers.insert(
        HeaderName::from_static("date"),
        HeaderValue::from_static("Sun, 05 Jan 2014 21:31:40 GMT"),
    );
    headers.insert(
        HeaderName::from_static("content-type"),
        HeaderValue::from_static("application/json"),
    );
    headers.insert(
        HeaderName::from_static("digest"),
        HeaderValue::from_static("SHA-256=X48E9qOokqqrvdts8nOJRJN3OWDUoyWxBf7kbu9DBPE="),
    );
    headers.insert(
        HeaderName::from_static("content-length"),
        HeaderValue::from_static("18"),
    );

    headers
}

#[test]
fn verify_all_signature() {
    let headers = construct_headers();

    let mut headers_signature = headers.clone();
    headers_signature.insert(
        HeaderName::from_static("signature"),
        HeaderValue::from_static(ALL_SIGNATURE_HEADER_VALUE),
    );

    let method = "post";
    let path = "/foo";
    let query = Some("param=value&pet=dog");

    wrap_cow!(Borrowed; method, path);
    wrap_cow!(Owned; headers_signature);
    wrap_cow_option!(Borrowed; query);

    let request = Request::new(
        method.clone(),
        path.clone(),
        query.clone(),
        headers_signature,
    );
    assert!(crate::verify(request, RSA_PUBLIC_KEY.as_bytes()).unwrap());

    let mut headers_authorization = headers;
    headers_authorization.insert(
        HeaderName::from_static("authorization"),
        HeaderValue::from_static(ALL_AUTHORIZATION_HEADER_VALUE),
    );

    wrap_cow!(Owned; headers_authorization);

    let request = Request::new(method, path, query, headers_authorization);
    assert!(crate::verify(request, RSA_PUBLIC_KEY.as_bytes()).unwrap());
}
/*
#[test]
fn create_basic_signature() {
    let headers = construct_headers();

    let request = Request::new("post", "/foo", Some("param=value&pet=dog"), &headers);

    let (header_name, header_value) = crate::sign(
        request,
        "Test",
        &["(request-target)", "host", "date"],
        RSA_PRIVATE_KEY.as_bytes(),
    )
    .unwrap();

    assert_eq!(header_name, "signature");
    assert_eq!(header_value, BASIC_SIGNATURE_HEADER_VALUE);

    let mut headers = construct_headers();
    headers.insert(header_name, header_value);

    let request = Request::new("post", "/foo", Some("param=value&pet=dog"), &headers);
    assert!(crate::verify(request, RSA_PUBLIC_KEY.as_bytes()).unwrap());
}
*/
#[test]
fn verify_basic_signature() {
    let headers = construct_headers();

    let mut headers_signature = headers.clone();
    headers_signature.insert(
        HeaderName::from_static("signature"),
        HeaderValue::from_static(BASIC_SIGNATURE_HEADER_VALUE),
    );

    let method = "post";
    let path = "/foo";
    let query = Some("param=value&pet=dog");

    wrap_cow!(Borrowed; method, path);
    wrap_cow!(Owned; headers_signature);
    wrap_cow_option!(Borrowed; query);

    let request = Request::new(
        method.clone(),
        path.clone(),
        query.clone(),
        headers_signature,
    );
    assert!(crate::verify(request, RSA_PUBLIC_KEY.as_bytes()).unwrap());

    let mut headers_authorization = headers;
    headers_authorization.insert(
        HeaderName::from_static("authorization"),
        HeaderValue::from_static(BASIC_AUTHORIZATION_HEADER_VALUE),
    );

    wrap_cow!(Owned; headers_authorization);

    let request = Request::new(method, path, query, headers_authorization);
    assert!(crate::verify(request, RSA_PUBLIC_KEY.as_bytes()).unwrap());
}
/*
#[test]
fn create_default_signature() {
    let headers = construct_headers();

    let request = Request::new("post", "/foo", Some("param=value&pet=dog"), &headers);

    let (header_name, header_value) =
        crate::sign(request, "Test", &["date"], RSA_PRIVATE_KEY.as_bytes()).unwrap();

    assert_eq!(header_name, "signature");
    assert_eq!(header_value, SIGNATURE_HEADER_VALUE);

    let mut headers = construct_headers();
    headers.insert(header_name, header_value);

    let request = Request::new("post", "/foo", Some("param=value&pet=dog"), &headers);
    assert!(crate::verify(request, RSA_PUBLIC_KEY.as_bytes()).unwrap());
}
*/
#[test]
fn verify_default_signature() {
    let headers = construct_headers();

    let mut headers_signature = headers.clone();
    headers_signature.insert(
        HeaderName::from_static("signature"),
        HeaderValue::from_static(SIGNATURE_HEADER_VALUE),
    );

    let method = "post";
    let path = "/foo";
    let query = Some("param=value&pet=dog");

    wrap_cow!(Borrowed; method, path);
    wrap_cow!(Owned; headers_signature);
    wrap_cow_option!(Borrowed; query);

    let request = Request::new(
        method.clone(),
        path.clone(),
        query.clone(),
        headers_signature,
    );
    assert!(crate::verify(request, RSA_PUBLIC_KEY.as_bytes()).unwrap());

    let mut headers_authorization = headers;
    headers_authorization.insert(
        HeaderName::from_static("authorization"),
        HeaderValue::from_static(AUTHORIZATION_HEADER_VALUE),
    );

    wrap_cow!(Owned; headers_authorization);

    let request = Request::new(method, path, query, headers_authorization);
    assert!(crate::verify(request, RSA_PUBLIC_KEY.as_bytes()).unwrap());
}
