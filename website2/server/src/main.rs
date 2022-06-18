#![feature(async_closure)]
#![feature(const_fn_trait_bound)]
#![allow(unused_parens)]

use std::{collections::HashSet, convert::Infallible, fs::File};

use actix_cors::Cors;
use actix_files::{Files, NamedFile};
use actix_web::{
    dev::Service,
    error, get,
    http::StatusCode,
    middleware, post,
    web::{self, Query},
    App, HttpRequest, HttpResponse, HttpServer, ResponseError, Result,
};
use app::{api::bing::BingSearchResult, routes::router};
use episcopal_api::{
    api::summary::DailySummary,
    calendar::Date,
    hymnal::{HymnMetadata, HymnNumber, Hymnal, Hymnals, EL_HIMNARIO, HYMNAL_1982, LEVAS, WLP},
    language::Language,
    library::{CommonPrayer, Library},
    liturgy::{Document, Slug, SlugPath},
};
use futures::StreamExt;
use lazy_static::lazy_static;
use leptos2::*;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use request_compat::RequestCompat;
use reqwest::header::CACHE_CONTROL;
use response_compat::ResponseCompat;
use serde::Deserialize;
use tempfile::tempdir;
use tokio::task::spawn_local;

mod bing;
mod request_compat;
mod response_compat;

const LOCALES: [&str; 1] = ["en"];

lazy_static! {
    pub static ref PROJECT_ROOT: String =
        std::env::var("PROJECT_ROOT").unwrap_or_else(|_| "..".to_string());
    pub static ref ROUTER: Router<app::routes::Index> = router();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "1234".to_string());
    // load TLS keys
    // to create a self-signed temporary cert for testing:
    // `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    // TODO migrate export_docx and video_search_api into app

    HttpServer::new(|| {
        App::new()
            //.wrap(middleware::Compress::default())
            .wrap(Cors::permissive())
            .app_data(web::FormConfig::default().limit(256 * 1024)) // increase max form size for DOCX export
            //.service(daily_summary)
            .service(export_docx)
            //.service(canticle_list_api)
            //.service(hymnal_api)
            //.service(hymnal_search_api)
            //.service(hymnal_search_api_with_metadata)
            .service(video_search_api)
            //.service(hymnal_word_cloud)
            .service(Files::new("/client", &format!("{}/client", *PROJECT_ROOT)))
            .service(
                web::scope("/static")
                    // cache settings for static files
                    .wrap_fn(|req, srv| {
                        let fut = srv.call(req);
                        async {
                            let mut res = fut.await?;
                            res.headers_mut().insert(
                                CACHE_CONTROL,
                                actix_web::http::header::HeaderValue::from_static(
                                    "max-age=31536000",
                                ),
                            );
                            Ok(res)
                        }
                    })
                    .wrap(middleware::Compress::default())
                    .service(Files::new("", &format!("{}/app/static", *PROJECT_ROOT))),
            )
            .default_service(
                web::route().to(async move |req: HttpRequest, body: web::Bytes, multipart: actix_multipart::Multipart| {
                    let req = RequestCompat::new(req, body.as_ref().to_vec());
                    let req = Arc::new(req) as Arc<dyn Request>;
                    if req.method() == http::Method::POST {
                        let res = ROUTER.post(&req).await;
                        match res {
                            ActionResponse::None => HttpResponse::NotFound().finish(),
                            ActionResponse::Response(res) => {
                                let res = ResponseCompat::from(res);
                                res.into()
                            }
                            ActionResponse::Error(e) => {
                                HttpResponse::InternalServerError().body(e.to_string())
                            }
                        }
                    } else {
                        let routed = ROUTER.get(&req).await;
                        let head_html = head(&routed);
                        let body = routed.body.html_stream();

                        let stream = futures::stream::once(async move { head_html })
                            .chain(body)
                            .chain(futures::stream::once(async {
                                "</body></html>".to_string()
                            }))
                            .map(|html| {
                                Ok(web::Bytes::from(html))
                                    as Result<web::Bytes, leptos2::router::RouterError>
                            });

                        HttpResponse::Ok()
                            .content_type("text/html")
                            .streaming(stream)
                    }
                }),
            )
    })
    .bind_openssl(&format!("{}:{}", host, port), builder)?
    .run()
    .await
}

fn stream_html(html: String) -> Result<web::Bytes, Infallible> {
    Ok(web::Bytes::from(html))
}

fn head(view: &RenderedView) -> String {
    format!("<!DOCTYPE html><html lang=\"{}\"><head><title>{}</title>{}{}<style>{}</style></head><body>", view.locale, view.title, meta(&view.meta), links(&view.links), styles(&view.styles))
}

fn meta(metas: &MetaTags) -> String {
    metas
        .iter()
        .map(|(name, content)| {
            if name == "charset" {
                format!("<meta charset=\"{}\">", content)
            } else {
                format!("<meta name=\"{}\" content=\"{}\">", name, content)
            }
        })
        .collect()
}

fn styles(styles: &Styles) -> String {
    let styles = styles.join("");
    minifier::css::minify(&styles)
        .map(|minified| minified.to_string())
        .unwrap_or(styles)
}

fn links(links: &[Node]) -> String {
    links.iter().map(|link| link.to_string()).collect()
}

// Word Cloud API Generator
#[get("/api/wordlist/hymnal/{hymnal}")]
async fn hymnal_word_cloud(hymnal: web::Path<String>) -> String {
    let hymnal = hymnal.into_inner();
    if hymnal == "all" {
        HYMNAL_1982
            .hymns
            .iter()
            .chain(LEVAS.hymns.iter())
            .chain(WLP.hymns.iter())
            .map(|hymn| hymn.text.as_str())
            .collect()
    } else {
        let hymnal = match hymnal.as_str() {
            "Hymnal1982" => &*HYMNAL_1982,
            "LEVAS" => &*LEVAS,
            "WLP" => &*WLP,
            _ => panic!("hymnal not found"),
        };
        hymnal.hymns.iter().map(|hymn| hymn.text.as_str()).collect()
    }
}

// JSON Daily Summary API
#[derive(Debug)]
pub struct DateError(episcopal_api::calendar::DateError);

impl std::fmt::Display for DateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl ResponseError for DateError {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

#[get("/api/daily_summary/{locale}/{date}.json")]
async fn daily_summary(params: web::Path<(String, String)>) -> Result<web::Json<DailySummary>> {
    let (locale, date) = params.into_inner();
    let date = Date::parse_from_str(&date, "%Y-%m-%d").map_err(DateError)?;
    let language = Language::from_locale(&locale);
    let summary = episcopal_api::library::CommonPrayer::daily_office_summary(&date, language);
    Ok(web::Json(summary))
}

// Canticle List API
#[get("/api/canticles.json")]
async fn canticle_list_api() -> Result<web::Json<Vec<Document>>> {
    let canticles = CommonPrayer::contents()
        .contents_at_path(&SlugPath::from([Slug::Office, Slug::Canticles]))
        .unwrap()
        .as_documents()
        .cloned()
        .collect();
    Ok(web::Json(canticles))
}

// Hymnal API
#[get("/api/hymnal/{hymnal}.json")]
async fn hymnal_api(path: web::Path<Hymnals>) -> Result<web::Json<Hymnal>> {
    Ok(web::Json(path.into_inner().into()))
}

// Hymnal Search API
#[derive(Deserialize, Debug)]
struct HymnalSearchParams {
    q: String,
    hymnal: Option<Hymnals>,
}

#[get("/api/hymnal/search")]
async fn hymnal_search_api(
    params: Query<HymnalSearchParams>,
) -> web::Json<HashSet<(Hymnals, HymnNumber)>> {
    let search = &params.q;
    let matches = if let Some(hymnal) = &params.hymnal {
        let hymnal = Hymnal::from(*hymnal);
        hymnal
            .search(search)
            .map(|hymn| (hymnal.id, hymn.number))
            .collect()
    } else {
        HYMNAL_1982
            .search(search)
            .map(|hymn| (Hymnals::Hymnal1982, hymn.number))
            .chain(
                LEVAS
                    .search(search)
                    .map(|hymn| (Hymnals::LEVAS, hymn.number)),
            )
            .chain(WLP.search(search).map(|hymn| (Hymnals::WLP, hymn.number)))
            .chain(
                EL_HIMNARIO
                    .search(search)
                    .map(|hymn| (Hymnals::ElHimnario, hymn.number)),
            )
            .collect()
    };

    web::Json(matches)
}

#[get("/api/hymnal/search_with_metadata")]
async fn hymnal_search_api_with_metadata(
    params: Query<HymnalSearchParams>,
) -> web::Json<Vec<HymnMetadata>> {
    let search = &params.q;
    let matches = if let Some(hymnal) = &params.hymnal {
        let hymnal = Hymnal::from(*hymnal);
        hymnal.search(search).collect()
    } else {
        HYMNAL_1982
            .search(search)
            .chain(LEVAS.search(search))
            .chain(WLP.search(search))
            .chain(EL_HIMNARIO.search(search))
            .collect()
    };

    web::Json(matches)
}

// Hymn Video API
#[derive(Deserialize, Debug)]
struct HymnVideoParams {
    hymnal: Hymnals,
    number: HymnNumber,
}

#[derive(Debug)]
pub struct HymnSearchError(reqwest::Error);

impl std::fmt::Display for HymnSearchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ResponseError for HymnSearchError {}

#[get("/api/hymnal/videos")]
async fn video_search_api(params: Query<HymnVideoParams>) -> Result<web::Json<BingSearchResult>> {
    let hymnal: Hymnal = params.hymnal.into();
    let hymn = hymnal
        .hymns
        .iter()
        .find(|hymn| hymn.number == params.number)
        .expect("could not find hymn with this #");

    let result = bing::search(hymn).await.map_err(HymnSearchError)?;
    Ok(web::Json(result))
}

#[derive(Deserialize)]
struct DocxExportFormData {
    liturgy: String,
    date: String,
    doc: String,
}

// Document Export APIs
#[post("/api/export/docx")]
async fn export_docx(data: web::Form<DocxExportFormData>) -> Result<NamedFile> {
    let data = data.into_inner();
    let doc: Document = serde_json::from_str(&data.doc)?;

    let slug = data.liturgy.replace('/', "-");
    let file_name = if !data.date.is_empty() {
        format!("{}-{}.docx", slug, data.date)
    } else {
        format!("{}.docx", slug)
    };
    let dir = tempdir()?;
    let path = dir.path().join(file_name);
    let file = File::create(&path)?;

    let docx = episcopal_api::docx::DocxDocument::from(doc);
    docx.write(&file)
        .map_err(|e| error::InternalError::new(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(NamedFile::open(path)?)
}
