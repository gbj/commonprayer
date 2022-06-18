use actix_web::HttpResponse;

pub struct ResponseCompat(Box<http::Response<()>>);

impl From<ResponseCompat> for actix_web::HttpResponse {
    fn from(res: ResponseCompat) -> Self {
        // status code
        let status_code: u16 = res.0.status().into();
        let mut builder = HttpResponse::build(
            actix_web::http::StatusCode::from_u16(status_code)
                .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR),
        );

        // headers
        for (header_name, header_value) in res.0.headers() {
            builder.append_header((
                header_name.to_string(),
                header_value
                    .to_str()
                    .expect("header value should only contain ASCII values")
                    .to_string(),
            ));
        }

        // body
        builder.finish()
    }
}

impl From<Box<http::Response<()>>> for ResponseCompat {
    fn from(res: Box<http::Response<()>>) -> Self {
        Self(res)
    }
}
