use actix_files::{Files, NamedFile};
use actix_web::{
    get,
    web::{self, Path},
    App, HttpRequest, HttpResponse, HttpServer, Result,
};
use episcopal_api::{api::summary::DailySummary, calendar::Date, liturgy::Document};
use lazy_static::lazy_static;
use leptos::Page;
use serde::{de::DeserializeOwned, Serialize};
use website::{pages::*, utils::language::locale_to_language};

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
            .service(json_document_export)
            .service(docx_document_export)
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

// Document Export APIs
#[get("/api/export/{category}/{version}/{date}/{calendar}/{prefs}/{alternate}/{slug}.json")]
async fn json_document_export(
    params: web::Path<DocumentPageParams>,
) -> Result<web::Json<Document>, ()> {
    // path is not used, and locale is only used for building the base URL at this point, so can be set to "" and "en"
    let props = website::pages::document::get_static_props("en", "", params.into_inner());
    Ok(web::Json(props.doc))
}

#[get("/api/export/{category}/{version}/{date}/{calendar}/{prefs}/{alternate}/{slug}.docx")]
async fn docx_document_export(params: web::Path<DocumentPageParams>) -> Result<HttpResponse, ()> {
    // path is not used, and locale is only used for building the base URL at this point, so can be set to "" and "en"
    let props = website::pages::document::get_static_props("en", "", params.into_inner());
    let docx = episcopal_api::docx::DocxDocument::from(props.doc);
    Ok(HttpResponse::Ok()
        .content_type("application/vnd.openxmlformats-officedocument.wordprocessingml.document")
        .body(docx.to_bytes()))
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

fn add_page<T, P>(cfg: &mut web::ServiceConfig, locale: &str, page: Page<T, P>)
where
    T: Serialize + DeserializeOwned + Clone + 'static,
    P: DeserializeOwned + Clone + 'static,
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
                    Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
                }
            }
        })));
    }
}
