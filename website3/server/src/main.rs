use std::pin::Pin;

use actix_files::{Directory, Files, NamedFile};
use actix_web::{dev::Service, http::header::CACHE_CONTROL, *};
use commonprayer_site::*;
use futures::{stream::FuturesUnordered, Future, StreamExt};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

#[derive(Copy, Clone, Debug)]
struct ActixIntegration {
    path: ReadSignal<String>,
}

impl History for ActixIntegration {
    fn location(&self, cx: leptos::Scope) -> ReadSignal<LocationChange> {
        create_signal(
            cx,
            LocationChange {
                value: self.path.get(),
                replace: false,
                scroll: true,
                state: State(None),
            },
        )
        .0
    }

    fn navigate(&self, loc: &LocationChange) {}
}

// match every path — our router will handle actual dispatch
#[get("{tail:.*}")]
async fn render_app(req: HttpRequest) -> impl Responder {
    let path = req.path();

    let query = req.query_string();
    let path = if query.is_empty() {
        "http://leptos".to_string() + path
    } else {
        "http://leptos".to_string() + path + "?" + query
    };

    let accepts_type = req.headers().get("Accept").map(|h| h.to_str());
    match accepts_type {
        // if asks for JSON, send the loader function JSON or 404
        Some(Ok("application/json")) => {
            log::debug!("\n\ngot JSON request at {path}");
            let (json, disposer) = run_scope_undisposed(move |cx| async move {
                let integration = ActixIntegration {
                    path: create_signal(cx, path).0,
                };
                provide_context(cx, RouterIntegrationContext(std::rc::Rc::new(integration)));

                let shell = view! { <App/> };

                let mut route = use_context::<leptos_router::RouteContext>(cx)?;
                // get the innermost route matched by this path
                while let Some(child) = route.child() {
                    route = child;
                }
                let data = route.loader().as_ref().map(|loader| loader.call(cx))?;

                data.await.serialize()
            });

            let res = if let Some(json) = json.await {
                HttpResponse::Ok()
                    .content_type("application/json")
                    .body(json)
            } else {
                HttpResponse::NotFound().body(())
            };

            disposer.dispose();
            res
        }
        // otherwise, send HTML
        _ => {
            log::debug!("GET {path}");

            let ((head, shell, pending_resources, pending_fragments, serializers), disposer) =
                run_scope_undisposed({
                    move |cx| {
                        let integration = ActixIntegration {
                            path: create_signal(cx, path).0,
                        };
                        provide_context(
                            cx,
                            RouterIntegrationContext(std::rc::Rc::new(integration)),
                        );

                        // the actual app body/template code
                        // this does NOT contain any of the data being loaded asynchronously in resources
                        let shell = view! { <App/> };

                        let mut head = String::from(
                            r#"<!DOCTYPE html>
            <html lang="en">
                <head>
                    <meta charset="utf-8"/>
                    <meta name="viewport" content="width=device-width, initial-scale=1"/>
                    <script type="module">import init, { main } from '/pkg/client.js'; init().then(main);</script>"#,
                        );

                        if let Some(meta) = use_context::<MetaContext>(cx) {
                            head.push_str(&meta.dehydrate());
                        }
                        head.push_str("</head><body>");

                        let resources = cx.all_resources();
                        let pending_resources = serde_json::to_string(&resources).unwrap();

                        (
                            head,
                            shell,
                            pending_resources,
                            cx.pending_fragments(),
                            cx.serialization_resolvers(),
                        )
                    }
                });

            let fragments = FuturesUnordered::new();
            for (fragment_id, fut) in pending_fragments {
                fragments.push(async move { (fragment_id, fut.await) })
            }

            HttpResponse::Ok().content_type("text/html").streaming(
                futures::stream::once(async move {
                    format!(
                        r#"
                {head}
                {shell}
                <script>
                    __LEPTOS_PENDING_RESOURCES = {pending_resources};
                    __LEPTOS_RESOLVED_RESOURCES = new Map();
                    __LEPTOS_RESOURCE_RESOLVERS = new Map();
                </script>
            "#
                    )
                })
                .chain(fragments.map(|(fragment_id, html)| {
                    format!(
                        r#"
                <template id="{fragment_id}">{html}</template>
                <script>
                    var frag = document.querySelector(`[data-fragment-id="{fragment_id}"]`);
                    var tpl = document.getElementById("{fragment_id}");
                    console.log("replace", frag, "with", tpl.content.cloneNode(true));
                    frag.replaceWith(tpl.content.cloneNode(true));
                </script>
                "#
                    )
                }))
                .chain(serializers.map(|(id, json)| {
                    let id = serde_json::to_string(&id).unwrap();
                    format!(
                        r#"<script>
                    if(__LEPTOS_RESOURCE_RESOLVERS.get({id})) {{
                        console.log("(create_resource) calling resolver");
                        __LEPTOS_RESOURCE_RESOLVERS.get({id})({json:?})
                    }} else {{
                        console.log("(create_resource) saving data for resource creation");
                        __LEPTOS_RESOLVED_RESOURCES.set({id}, {json:?});
                    }}
                    </script>"#,
                    )
                }))
                .chain(futures::stream::once(async {
                    "</body></html>".to_string()
                }))
                .map(|html| Ok(web::Bytes::from(html)) as Result<web::Bytes>),
            )

            // TODO handle disposer; currently leaking memory from scope
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

    log::debug!("serving at {host}:{port}");

    // load TLS keys
    // to create a self-signed temporary cert for testing:
    // `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/pkg")
                    .service(Files::new("", "../client/pkg"))
                    .wrap(middleware::Compress::default()),
            )
            .service(
                web::scope("/static")
                    .service(Files::new("", "../commonprayer-site/static"))
                    .wrap(middleware::Compress::default()),
            )
            .service(
                web::scope("/styles")
                    .service(Files::new("", "../commonprayer-site/styles"))
                    .wrap(middleware::Compress::default()),
            )
            .service(render_app)
        //.wrap(middleware::Compress::default())
        /* .service(
            web::scope("/pkg")
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
                .service(Files::new("", "../hackernews-client/pkg")),
        ) */
    })
    .bind(("127.0.0.1", 8080))?
    //.bind_openssl(&format!("{}:{}", host, port), builder)?
    .run()
    .await
}
