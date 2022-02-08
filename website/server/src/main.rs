use std::{fs::File, collections::HashSet};

use actix_files::{Files, NamedFile};
use actix_web::{
    error, get,
    http::StatusCode,
    web::{self, Path, Query},
    App, FromRequest, HttpRequest, HttpResponse, HttpServer, Result,
};
use episcopal_api::{
    api::summary::DailySummary,
    calendar::Date,
    hymnal::{Hymnal, Hymnals, HymnNumber, HYMNAL_1982, LEVAS, WLP},
    liturgy::Document,
};
use lazy_static::lazy_static;
use leptos::Page;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tempfile::tempdir;
use website::{pages::*, utils::language::locale_to_language, PageType, TABLE_OF_CONTENTS};

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
            .service(daily_summary)
            .service(
                web::resource("/api/export/docx")
                    .route(web::post().to(export_docx))
                    .data(web::Form::<DocxExportFormData>::configure(|cfg| {
                        cfg.limit(256 * 1024)
                    })),
            )
            .service(canticle_list_api)
            .service(hymnal_api)
            .service(hymnal_search_api)
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
async fn hymnal_word_cloud(hymnal : web::Path<String>) -> String {
    let hymnal = hymnal.into_inner();
    if hymnal == "all" {
        HYMNAL_1982.hymns.iter().chain(LEVAS.hymns.iter()).chain(WLP.hymns.iter()).map(|hymn| hymn.text.as_str()).collect()
    } else {
        let hymnal = match hymnal.as_str() {
            "Hymnal1982" => &*HYMNAL_1982,
            "LEVAS" => &*LEVAS,
            "WLP" => &*WLP,
            _ => panic!("hymnal not found")
        };
        hymnal.hymns.iter().map(|hymn| hymn.text.as_str()).collect()
    }
}

// JSON Daily Summary API
#[get("/api/daily_summary/{locale}/{date}.json")]
async fn daily_summary(
    web::Path((locale, date)): web::Path<(String, String)>,
) -> Result<web::Json<DailySummary>, ()> {
    let date = Date::parse_from_str(&date, "%Y-%m-%d").map_err(|_| ())?;
    let language = locale_to_language(&locale);
    let summary = episcopal_api::library::CommonPrayer::summarize_date(&date, language);
    Ok(web::Json(summary))
}

// Canticle List API
#[get("/api/canticles.json")]
async fn canticle_list_api() -> Result<web::Json<Vec<Document>>, ()> {
    let canticles = TABLE_OF_CONTENTS
        .get("canticle")
        .unwrap()
        .iter()
        .filter_map(|(_, page_type)| {
            if let PageType::Document(doc) = page_type {
                Some(doc.clone())
            } else {
                None
            }
        })
        .collect();
    Ok(web::Json(canticles))
}

// Hymnal API
#[get("/api/hymnal/{hymnal}.json")]
async fn hymnal_api(path: web::Path<Hymnals>) -> Result<web::Json<Hymnal>, ()> {
    Ok(web::Json(path.into_inner().into()))
}

// Hymnal Search API
#[derive(Deserialize)]
struct HymnalSearchParams {
    q: String
}

#[get("/api/hymnal/search")]
async fn hymnal_search_api(params: Query<HymnalSearchParams>) -> web::Json<HashSet<(Hymnals, HymnNumber)>> {
    let search = strip_non_word_characters(&params.q.to_lowercase());
    let matches = HYMNAL_1982.hymns.iter().chain(LEVAS.hymns.iter()).chain(WLP.hymns.iter()).filter(|hymn| {
        let number = strip_non_word_characters(&hymn.number.to_string().to_lowercase());
                                        let title = strip_non_word_characters(&hymn.title.to_lowercase());
                                        let tune = strip_non_word_characters(&hymn.tune.to_lowercase());
                                        let authors = strip_non_word_characters(&hymn.authors.to_lowercase());
                                        let composers = strip_non_word_characters(&hymn.composers.to_lowercase());
                                        let text = strip_non_word_characters(&hymn.text.to_lowercase());
                                            number.contains(&search)
                                                || title.contains(&search)
                                                || tune.contains(&search)
                                                || authors.contains(&search)
                                                || composers.contains(&search)
                                                || hymn.meter.contains(&params.q)
                                                || text.contains(&search)
                                            
                                        
    }).map(|hymn| (hymn.source, hymn.number)).collect::<HashSet<_>>();
    web::Json(matches)
}

fn strip_non_word_characters(original: &str) -> String {
    original.chars().filter(|ch| 
        // so that date ranges don't get read as numbers, i.e., "111" should not match "1711-1759"
        ch == &'-'
        // letters
        || ('a'..'z').contains(ch)
        // digits so can search by hymn number
        || ('0'..'9').contains(ch)).collect()
}

#[derive(Deserialize)]
struct DocxExportFormData {
    liturgy: String,
    date: String,
    doc: String,
}

// Document Export APIs
async fn export_docx(data: web::Form<DocxExportFormData>) -> Result<NamedFile> {
    let data = data.into_inner();
    let doc: Document = serde_json::from_str(&data.doc)?;

    let file_name = if !data.date.is_empty() {
        format!("{}-{}.docx", data.liturgy, data.date)
    } else {
        format!("{}.docx", data.liturgy)
    };
    let dir = tempdir()?;
    let path = dir.path().join(file_name);
    let file = File::create(&path)?;

    let mut docx = episcopal_api::docx::DocxDocument::from(doc);
    docx.write(&file)
        .map_err(|e| error::InternalError::new(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(NamedFile::open(path)?)
}

// Add additional pages, defined programmatically
fn add_pages(cfg: &mut web::ServiceConfig, locales: &[&str]) {
    for locale in locales {
        add_page(cfg, locale, calendar());
        add_page(cfg, locale, canticle_table());
        add_page(cfg, locale, daily_office());
        add_page(cfg, locale, daily_readings());
        add_page(cfg, locale, document());
        add_page(cfg, locale, holy_day());
        add_page(cfg, locale, hymnal());
        add_page(cfg, locale, index());
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
    R: Default + Clone + 'static
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
                |req: HttpRequest| {
                    let preferred_locale = req
                        .headers()
                        .get("Accept-Language")
                        .and_then(|locale| locale.to_str().unwrap().split('-').next())
                        .unwrap_or("en");
                    let path = req.path();
                    HttpResponse::TemporaryRedirect()
                        .set_header(
                            actix_web::http::header::LOCATION,
                            format!(
                                "/{}{}",
                                preferred_locale,
                                if path == "/" { "" } else { path }
                            ),
                        )
                        .finish()
                },
            )),
        );

        // The page
        cfg.service(web::resource(&localized_path).route(web::get().to({
            let locale = locale.to_string();
            let page = page.clone();
            move |req: HttpRequest, params: Path<P>| {
                let path = req.path();
                match page.build(&locale, path, params.into_inner()) {
                    Ok(view) => HttpResponse::Ok().body(&view.to_html()),
                    Err(leptos::PageRenderError::NotFound) => {
                        let not_found = not_found_404().build(&locale, path, ()).unwrap();
                        HttpResponse::Ok().body(&not_found.to_html())
                    }
                    Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
                }
            }
        })));
    }
}
