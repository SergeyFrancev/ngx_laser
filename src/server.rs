use std::convert::Infallible;

use cli_log::debug;
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};

use crate::{conf, LogBase};

use std::path::PathBuf;

async fn hello(request: Request<Body>) -> Result<Response<Body>, Infallible> {
    debug!("=================================");
    debug!("Req uri: {}", request.uri().path());
    let conf = conf::get().lock().unwrap();
    let mut paths = conf.base_dir.clone();
    let res = LogBase::new(&[paths]);
    if res.is_ok() {
        return Ok(Response::new(Body::from(format!("Counter: {}", res.unwrap().lines.len()))));
    } else {
        return Ok(Response::new(Body::from(format!("Error: {}", res.err().unwrap()))));
    }
    // let path = request.uri().path();
    // match serve_file(&path).await {
    //     Ok(file_content) => Ok(Response::new(Body::from(file_content))),
    //     Ok(_) => Ok(Response::new(Body::from(format!("Path: {}", path)))),
    //     Err(_) => Ok(Response::builder().status(404).body(Body::empty()).unwrap()),
    // }
    Ok(Response::new(Body::from(format!("Path: {}", "123"))))
}

#[tokio::main]
pub async fn start(port: u16) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(hello)) });
    let addr = ([127, 0, 0, 1], port).into();
    let server = Server::bind(&addr).serve(make_svc);
    println!("Listening on http://{}", addr);
    server.await?;
    Ok(())
}
