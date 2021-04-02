use {
    crate::{
        error::{Error, Result},
        request::Request,
        util::{HeaderMapExt as _, IteratorExt as _},
    },
    http::header::DATE,
};

#[derive(Debug, PartialEq)]
pub enum Part<'a> {
    /// Header with key and value
    Header(&'a str, &'a str),

    /// Parts needed for the request target (method, path and query)
    RequestTarget(&'a str, &'a str, Option<&'a str>),
}

impl<'a> ToString for Part<'a> {
    fn to_string(&self) -> String {
        match self {
            Part::Header(key, value) => format!("{}: {}", key, value),
            Part::RequestTarget(method, path, query) => {
                let method = method.to_lowercase();
                let query = query.map(|query| format!("?{}", query)).unwrap_or_default();

                format!("(request-target): {} {}{}", method, path, query)
            }
        }
    }
}

pub struct SignatureString<'a> {
    parts: Vec<Part<'a>>,
}

impl<'a> SignatureString<'a> {
    /// Build a new signature string
    pub fn build(request: &'a Request<'a>, requested_fields: &'a [&str]) -> Result<Self> {
        let mut parts = requested_fields
            .iter()
            // The `created` and `expires` fields shouldn't be part of the signature string
            .filter(|field| **field != "(created)" && **field != "(expires)")
            .map(|field| {
                let method = request.method;
                let path = request.path;
                let query = request.query;

                let part = match *field {
                    "(request-target)" => Part::RequestTarget(method, path, query),
                    header_name => {
                        let header_value = request.headers.get_header(header_name)?;
                        let header_value = header_value.to_str()?;

                        Part::Header(header_name, header_value)
                    }
                };

                Ok::<_, Error>(part)
            })
            .try_collect_vec()?;

        // If a list of headers isn't included, only the "date" header is used
        // See draft-cavage-http-signatures-11#Appendix.C.1
        if parts.is_empty() {
            let header_value = request.headers.get_header(DATE)?;
            let header_value = header_value.to_str()?;

            parts.push(Part::Header("date", header_value));
        }

        let signature_string = SignatureString { parts };
        Ok(signature_string)
    }
}

impl<'a> ToString for SignatureString<'a> {
    fn to_string(&self) -> String {
        self.parts
            .iter()
            .map(ToString::to_string)
            .collect_vec()
            .join("\n")
    }
}
