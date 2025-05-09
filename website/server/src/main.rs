#![feature(async_closure)]
#![feature(const_fn_trait_bound)]

use std::{collections::HashSet, fs::File, io::Write};

use actix_cors::Cors;
use actix_files::{Files, NamedFile};
use actix_web::{
    error, get,
    http::StatusCode,
    post,
    web::{self, Path, Query},
    App, HttpRequest, HttpResponse, HttpServer, ResponseError, Result,
};
use episcopal_api::{
    api::summary::DailySummary,
    calendar::Date,
    hymnal::{HymnNumber, Hymnal, Hymnals, HYMNAL_1982, LEVAS, WLP, EL_HIMNARIO},
    liturgy::{Document, SlugPath, Slug}, library::{CommonPrayer, Library},
};
use lazy_static::lazy_static;
use leptos::{view, Page};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tempfile::tempdir;
use website::{
    api::bing::BingSearchResult, pages::*, utils::language::locale_to_language
};

mod bing;

const LOCALES: [&str; 1] = ["en"];

lazy_static! {
    pub static ref PROJECT_ROOT: String =
        std::env::var("PROJECT_ROOT").unwrap_or_else(|_| "..".to_string());
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "1234".to_string());

    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .app_data(web::FormConfig::default().limit(256 * 1024)) // increase max form size for DOCX export
            .service(daily_summary)
            .service(export_docx)
            .service(canticle_list_api)
            .service(hymnal_api)
            .service(hymnal_search_api)
            .service(video_search_api)
            .service(hymnal_word_cloud)
            .service(Files::new(
                "/static",
                &format!("{}/website/static", *PROJECT_ROOT),
            ))
            .configure(|cfg| add_pages(cfg, &LOCALES))
    })
    .bind(&format!("{}:{}", host, port))?
    .run()
    .await
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
    let language = locale_to_language(&locale);
    let summary = episcopal_api::library::CommonPrayer::daily_office_summary(&date, language);
    Ok(web::Json(summary))
}

#[get("/api/eucharistic_summary/{locale}/{date}.json")]
async fn eucharistic_summary(params: web::Path<(String, String)>) -> Result<web::Json<DailySummary>> {
    let (locale, date) = params.into_inner();
    let date = Date::parse_from_str(&date, "%Y-%m-%d").map_err(DateError)?;
    let language = locale_to_language(&locale);
    let summary = episcopal_api::library::CommonPrayer::eucharistic_lectionary_summary(&date, language);
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
#[derive(Deserialize)]
struct HymnalSearchParams {
    q: String,
}

#[get("/api/hymnal/search")]
async fn hymnal_search_api(
    params: Query<HymnalSearchParams>,
) -> web::Json<HashSet<(Hymnals, HymnNumber)>> {
    let search = &params.q;
    let matches = HYMNAL_1982
        .search(search)
        .map(|hymn| (Hymnals::Hymnal1982, hymn.number))
        .chain(LEVAS.search(search).map(|hymn| (Hymnals::LEVAS, hymn.number)))
        .chain(WLP.search(search).map(|hymn| (Hymnals::WLP, hymn.number)))
        .chain(EL_HIMNARIO.search(search).map(|hymn| (Hymnals::ElHimnario, hymn.number)))
        .collect();

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
        .find(|hymn| hymn.number == params.number).expect("could not find hymn with this #");

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

// Add additional pages, defined programmatically
fn add_pages(cfg: &mut web::ServiceConfig, locales: &[&str]) {
    // check if static export directory exists
    let artifacts_path = format!("{}/artifacts", *PROJECT_ROOT);
    let artifacts_path = std::path::Path::new(&artifacts_path);
    if artifacts_path.exists() {
        println!("emptying artifacts directory");
        std::fs::remove_dir_all(artifacts_path).expect("couldn't empty artifacts directory");
        std::fs::create_dir(artifacts_path).expect("couldn't create artifacts directory");
    } else {
        println!("creating artifacts directory");
        std::fs::create_dir(artifacts_path).expect("could not create artifacts directory");
    }

    // add pages
    for locale in locales {
        add_page(cfg, locale, about());
        add_page(cfg, locale, calendar());
        add_page(cfg, locale, canticle_table());
        add_page(cfg, locale, daily_office());
        add_page(cfg, locale, readings());
        add_page(cfg, locale, document());
        add_page(cfg, locale, holy_day());
        add_page(cfg, locale, hymnal());
        add_page(cfg, locale, index());
        add_page(cfg, locale, lectionary());
        add_page(cfg, locale, psalter());
        add_page(cfg, locale, settings());
    }
}

async fn serve_page_js(name: String) -> Result<NamedFile> {
    Ok(NamedFile::open(&format!(
        "{}/client/{}/pkg/{}_page.js",
        *PROJECT_ROOT, name, name
    ))?)
}

async fn serve_page_wasm(name: String) -> Result<NamedFile> {
    Ok(NamedFile::open(&format!(
        "{}/client/{}/pkg/{}_page_bg.wasm",
        *PROJECT_ROOT, name, name
    ))?)
}

fn add_page<T, P, R>(cfg: &mut web::ServiceConfig, locale: &str, page: Page<T, P, R>)
where
    T: Serialize + DeserializeOwned + Clone + 'static,
    P: DeserializeOwned + Clone + 'static,
    R: Default + Clone + 'static,
{
    println!("{}", page.name);
    for path in page.get_absolute_paths() {
        // "index" is special case that uses / instead
        let localized_path = if path == "index" {
            locale.to_string()
        } else {
            format!("{locale}/{path}")
        };
        println!("\t{}", path);

        // JS/WASM routes for the page
        cfg.service(
            web::resource(&format!("/pkg/{}_page.js", page.name.replace('-', "_")))
                .route(web::get().to(|| serve_page_js(page.name.replace('-', "_")))),
        );
        cfg.service(
            web::resource(&format!(
                "/pkg/{}_page_bg.wasm",
                page.name.replace('-', "_")
            ))
            .route(web::get().to(|| serve_page_wasm(page.name.replace('-', "_")))),
        );

        // Page with no locale => redirect based on preferred locale in request
        cfg.service(
            web::resource(if path == "index" { "/" } else { path.as_str() }).route(web::get().to(
                async move |req: HttpRequest| {
                    let preferred_locale = req
                        .headers()
                        .get("Accept-Language")
                        .and_then(|locale| locale.to_str().unwrap().split('-').next())
                        .unwrap_or("en");
                    let path = req.path();
                    HttpResponse::TemporaryRedirect()
                        .insert_header((
                            actix_web::http::header::LOCATION,
                            format!(
                                "/{}{}",
                                preferred_locale,
                                if path == "/" { "" } else { path }
                            ),
                        ))
                        .finish()
                },
            )),
        );

        // The page
        cfg.service(web::resource(&localized_path).route(web::get().to({
            let locale = locale.to_string();
            let page = page.clone();

            move |req: HttpRequest, params: Path<P>| {
                let page = page.clone();
                let locale = locale.to_string();
                
                async move {
                    let path = req.uri().to_string();

                    // Plausible.io is an open-source analytics software as a service that uses no cookies and collects/sells no user data
                    // It is an alternative to Google Analytics, etc. with strong privacy protections
                    // Rather than an advertising based model, I pay a subscription fee to support their service
                    let analytics_injection = view! {
                        <script defer data-domain="commonprayeronline.org" src="https://plausible.io/js/plausible.js"></script>
                    };

                    // if incremental generation, check if page has already been created and serve that file if it has
                    if page.incremental_generation {
                        let build_artifact_dir = [(*PROJECT_ROOT).clone(), "artifacts".to_string()].into_iter().chain(path.split('/').map(String::from)).collect::<Vec<_>>().join("/");
                        let mut build_artifact_path = build_artifact_dir.clone();
                        build_artifact_path.push_str(".html");
                        let build_artifact_path = std::path::Path::new(&build_artifact_path);
                        if build_artifact_path.exists() {
                            match NamedFile::open(build_artifact_path) {
                                Ok(file) => file.into_response(&req),
                                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
                            }
                        } else {
                            let build_artifact_dir_path = std::path::Path::new(&build_artifact_dir);
                            if !build_artifact_dir_path.exists() {
                                std::fs::create_dir_all(build_artifact_dir_path).expect("could not create static file dir");
                            }

                            let mut file = File::create(build_artifact_path).expect("could not create static file");
                            match &page.build(&locale, &path, params.into_inner(), Some(analytics_injection)) {
                                Ok(view) => {
                                    let html= view.to_html();
                                    match file.write(html.as_bytes()) {
                                        Ok(_) => println!("\t\t\tgenerated static file at {}", path),
                                        Err(e) => println!("\t\t\terror generating static file for {}\n\t\t\t\t{}", path, e)
                                    };
                                    HttpResponse::Ok().body(html)
                                },
                                Err(leptos::PageRenderError::NotFound) => {
                                    let not_found = not_found_404().build(&locale, &path, (), None).unwrap();
                                    HttpResponse::Ok().body(not_found.to_html())
                                }
                                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
                            }
                        }
                    }
                    // otherwise, just render and serve the page
                    else {
                        match &page.build(&locale, &path, params.into_inner(), Some(analytics_injection)) {
                            Ok(view) => HttpResponse::Ok().body(view.to_html()),
                            Err(leptos::PageRenderError::NotFound) => {
                                let not_found = not_found_404().build(&locale, &path, (), None).unwrap();
                                HttpResponse::Ok().body(not_found.to_html())
                            }
                            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
                        }
                    }
                }
            }
        })));
    }
}
