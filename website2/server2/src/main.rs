use std::collections::HashMap;

use app::routes::router;
use leptos2::Router;
use tokio::{spawn, task::spawn_local};
use warp::{
    hyper::{body::Bytes, Body, Response, StatusCode},
    Filter,
};

lazy_static::lazy_static! {
    pub static ref PROJECT_ROOT: String =
        std::env::var("PROJECT_ROOT").unwrap_or_else(|_| "..".to_string());

    pub static ref ROUTER: Router<app::routes::Index> = router();
}

#[tokio::main]
async fn main() {
    //let routed = ROUTER.route(&RequestCompat(req)).await;

    //    std::thread::sleep(std::time::Duration::from_secs(2));
    //    std::thread::sleep(std::time::Duration::from_secs(2));
    //    std::thread::sleep(std::time::Duration::from_secs(2));
    //   std::thread::sleep(std::time::Duration::from_secs(2));

    /* let (tx, rx) = unbounded::<String>();
    let cursor = Cursor::new(String::from("<html><p>Hello, Greg!</p>"));
    spawn_local({ let mut cursor = cursor.clone(); async move {
        cursor.set_position(0);
        cursor.get_mut().push_str("<p>Another paragraph</p>");
    }});
    Ok(Response::builder(200)
    .content_type(mime::HTML)
    .body(Body::from_reader(rx, None))
    .build()) */

    let example1 = warp::get().and(warp::path("example")).map(|| {
        let (mut tx, body) = Body::channel();
        spawn(async move {
            tx.send_data(Bytes::from("<p>Paragraph one</p>"));
            std::thread::sleep(std::time::Duration::from_secs(3));
            tx.send_data(Bytes::from("<p>Paragraph two</p>"));
        });
        Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "text/html")
            .body(body)
    });

    warp::serve(example1).run(([127, 0, 0, 1], 3030)).await
}

/* struct RequestCompat(Request<()>);

impl leptos2::Request for RequestCompat {
    fn path(&self) -> &str {
        self.0.url().path()
    }

    fn query_string(&self) -> &str {
        self.0.url().query().unwrap_or_default()
    }
} */
