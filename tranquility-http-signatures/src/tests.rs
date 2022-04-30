use crate::Request;
use http::{header::HeaderName, HeaderMap, HeaderValue};

const RSA_PUBLIC_KEY: &str = r#"
-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDCFENGw33yGihy92pDjZQhl0C3
6rPJj+CvfSC8+q28hxA161QFNUd13wuCTUcq0Qd2qsBe/2hFyc2DCJJg0h1L78+6
Z4UMR7EOcpfdUE9Hf3m/hs+FUR45uBJeDK1HSFHD8bHKD6kv8FPGfJTotc+2xjJw
oYi+1hqp1fIekaxsyQIDAQAB
-----END PUBLIC KEY-----
"#;

// We use different keys here because the official test values use 1024 bit keys and
// ring refuses to sign messages with keys that small
const CREATE_RSA_PUBLIC_KEY: &str = r#"
-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAuLs+YOuVhy+j80AH08Ok
+7oXsar3EYgBv23vMxTQwjZ198H237gFy86fh7qV8TgT/249YhaEYqnJWbJK97Ih
X8QxONH9F8cQC43Nka7566jsBPMgnlIrIwgQLAAD9e7aNtrZpuYE/UqUMuSctkJO
maK95T+OcHnY2LSaZijvQJ4wUEhjwpi1+8/+Qs3CbPineUp2MCownI1g6t8TgxLW
yfslPkwVh8R/Uv8p/rkYincIqF595RsxE5lAa0ca7Vm5j2nq6b3pcujgpjUorJnG
JPIPIPkdL0EJd0EeJJqTFz1Cv8B7XFnMQtx1DLdVvKhEc08wjBHTprDTQ5NZA7dr
RQIDAQAB
-----END PUBLIC KEY-----
"#;

const CREATE_RSA_PRIVATE_KEY: &str = r#"
-----BEGIN RSA PRIVATE KEY-----
MIIEpAIBAAKCAQEAuLs+YOuVhy+j80AH08Ok+7oXsar3EYgBv23vMxTQwjZ198H2
37gFy86fh7qV8TgT/249YhaEYqnJWbJK97IhX8QxONH9F8cQC43Nka7566jsBPMg
nlIrIwgQLAAD9e7aNtrZpuYE/UqUMuSctkJOmaK95T+OcHnY2LSaZijvQJ4wUEhj
wpi1+8/+Qs3CbPineUp2MCownI1g6t8TgxLWyfslPkwVh8R/Uv8p/rkYincIqF59
5RsxE5lAa0ca7Vm5j2nq6b3pcujgpjUorJnGJPIPIPkdL0EJd0EeJJqTFz1Cv8B7
XFnMQtx1DLdVvKhEc08wjBHTprDTQ5NZA7drRQIDAQABAoIBAAI7gQ16y1vO35RY
+b89ZgkEvrSO9F1p31uI4JMldUBjmBleZrVda/SCkrr5Lkaz/snfcy50RzVKB/zl
grJrnKujm1SsdPqMlU0OcaWJD8whRnjo10QSiiLqPgfKGEIomMqA6pBxxy2ocRIM
YThypLCGnvTeR8JkNpKn3BXP+NQ3D+RRZ0aPYdzFS9ksAoT4B7zYV6uHG1fE3X5E
iJafRsvRc7TnNpf2C1rWRaDN6kaH7+N2C93a+EGw/2ieiAKrejEzpopP58Leehsh
Jq7AV9t3Fo5x8T0xBBIWEqVz9etUZnzbxiK6nxfbcF0rR2oT+Gqv14SbxQNj3ziF
44V/lAECgYEA81tuMCtVg1H/moWdCp0mkvrNEcfutUO4GlSaVyUFjnzKyhSXZhS9
cjp3AX4ejQXXkJFyqWiVJOZydMA4gbUMexIIpmpm9/8uWdLhj1m0tTOIsDGW8wei
tph6X2V3BG6oEHuZtswtvTvAv2T1AlQ93fyoQEeIqC6SVzt6BB8e+eECgYEAwlQc
RbQkB/J9cSnXmw0Ono36o8156FECHslUnuclY77i7xapNi5LzCobZtyMnsSN4QLm
4CeRAFOIs99wqFjKYS3qZsWpBT3+vJv4ghUmKZITYlF8IBdf2usDRqhRA0BmED4/
qJpFVAt2JKaU4nwCKNY+yqCQZzcvvdfm5JswheUCgYEA7yZmvoeXXZnzalLr5UZS
ZhZ5+INWHmQSRC3oDsOfFkukrbpWnka7dcnmsVzTgRrAoJ2O5NSV3NFqoTlVToIx
ZbBvN7tQvV3UmwkWCN3LLFcceKoDAYn7aR2nBKCduYlVN/1/LZixSkmyPWRlMoi+
06w7XA/wR/accYVNf0dmFYECgYB6ovggqRmgBlFR7DULvca/GxzU6OSJTy5GXYpQ
qdD3zMyMVEG/VqIxG1WlqYP44lQjb2Bij7W7ffwkf9sp8rbtczudVhpfm4s6XjgL
Z+toiq4++uuZmQa+Mlgj7C8MHUUL9SzZa1pbOsx5PsNw1w/J08NWvtPCv5oadbla
BfIuXQKBgQDGeVeID9TU5ipApZlLY+QJsC1vpFhbYZLValGt4r4zfSP2vbWUZMat
t2mKXVx9aQFMWSeagqKU9RS6yze/R1q+cqpNXoHzBgacrlC5K3ao3G67+RVgmO0L
QzqJYV+3js221+oZcaGm5Md97T6Os8NC0n16XLPVALGEyugme+odJw==
-----END RSA PRIVATE KEY-----
"#;

const SIGNATURE_HEADER_VALUE: &str = r#"keyId="Test",algorithm="rsa-sha256",signature="SjWJWbWN7i0wzBvtPl8rbASWz5xQW6mcJmn+ibttBqtifLN7Sazz6m79cNfwwb8DMJ5cou1s7uEGKKCs+FLEEaDV5lp7q25WqS+lavg7T8hc0GppauB6hbgEKTwblDHYGEtbGmtdHgVCk9SuS13F0hZ8FD0k/5OxEPXe5WozsbM=""#;
const AUTHORIZATION_HEADER_VALUE: &str = r#"Signature keyId="Test",algorithm="rsa-sha256",signature="SjWJWbWN7i0wzBvtPl8rbASWz5xQW6mcJmn+ibttBqtifLN7Sazz6m79cNfwwb8DMJ5cou1s7uEGKKCs+FLEEaDV5lp7q25WqS+lavg7T8hc0GppauB6hbgEKTwblDHYGEtbGmtdHgVCk9SuS13F0hZ8FD0k/5OxEPXe5WozsbM=""#;

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

    let request = Request::new(method, path, query, &headers_signature);
    assert!(crate::verify(request, RSA_PUBLIC_KEY.as_bytes()).unwrap());

    let mut headers_authorization = headers;
    headers_authorization.insert(
        HeaderName::from_static("authorization"),
        HeaderValue::from_static(ALL_AUTHORIZATION_HEADER_VALUE),
    );

    let request = Request::new(method, path, query, &headers_authorization);
    assert!(crate::verify(request, RSA_PUBLIC_KEY.as_bytes()).unwrap());
}

#[test]
fn create_basic_signature() {
    let headers = construct_headers();

    let request = Request::new("post", "/foo", Some("param=value&pet=dog"), &headers);

    let (header_name, header_value) = crate::sign(
        request,
        &["(request-target)", "host", "date"],
        ("Test", CREATE_RSA_PRIVATE_KEY.as_bytes()),
    )
    .unwrap();

    let mut headers = construct_headers();
    headers.insert(header_name, header_value);

    let request = Request::new("post", "/foo", Some("param=value&pet=dog"), &headers);
    assert!(crate::verify(request, CREATE_RSA_PUBLIC_KEY.as_bytes()).unwrap());
}

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

    let request = Request::new(method, path, query, &headers_signature);
    assert!(crate::verify(request, RSA_PUBLIC_KEY.as_bytes()).unwrap());

    let mut headers_authorization = headers;
    headers_authorization.insert(
        HeaderName::from_static("authorization"),
        HeaderValue::from_static(BASIC_AUTHORIZATION_HEADER_VALUE),
    );

    let request = Request::new(method, path, query, &headers_authorization);
    assert!(crate::verify(request, RSA_PUBLIC_KEY.as_bytes()).unwrap());
}

#[test]
fn create_default_signature() {
    let headers = construct_headers();

    let request = Request::new("post", "/foo", Some("param=value&pet=dog"), &headers);

    let (header_name, header_value) = crate::sign(
        request,
        &["date"],
        ("Test", CREATE_RSA_PRIVATE_KEY.as_bytes()),
    )
    .unwrap();

    let mut headers = construct_headers();
    headers.insert(header_name, header_value);

    let request = Request::new("post", "/foo", Some("param=value&pet=dog"), &headers);
    assert!(crate::verify(request, CREATE_RSA_PUBLIC_KEY.as_bytes()).unwrap());
}

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

    let request = Request::new(method, path, query, &headers_signature);
    assert!(crate::verify(request, RSA_PUBLIC_KEY.as_bytes()).unwrap());

    let mut headers_authorization = headers;
    headers_authorization.insert(
        HeaderName::from_static("authorization"),
        HeaderValue::from_static(AUTHORIZATION_HEADER_VALUE),
    );

    let request = Request::new(method, path, query, &headers_authorization);
    assert!(crate::verify(request, RSA_PUBLIC_KEY.as_bytes()).unwrap());
}
