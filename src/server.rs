use std::convert::Infallible;

use cli_log::debug;
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use serde_json::json;

use crate::{conf, Consumers, FileFinder, FormatVisitor, LogFile, Parser};

// static GLOBAL_CONFIG: OnceCell<Mutex<Config>> = OnceCell::new();

async fn hello(request: Request<Body>) -> Result<Response<Body>, Infallible> {
    debug!("=================================");
    debug!("Req uri: {}", request.uri().path());
    let conf = conf::get().lock().unwrap();
    let paths = [conf.base_dir.clone()];
    // let res = LogBase::new(&[paths], &conf.format);
    let parser = Parser::new(conf.format.as_str()).unwrap();

    // let mut ua: HashSet<String> = HashSet::new();
    // let paths = [PathBuf::from("/Users/nulldata/Documents/projects/rust/file_db/test_data/big/")];
    let mut file_finder = FileFinder::new(&paths, &parser);

    let mut consumers = Consumers::new();
    file_finder
        .dated_files()
        .unwrap()
        .into_iter()
        .map(|x| x.1)
        .flat_map(|x| LogFile::new(&x, &parser).into_iter())
        .for_each(|x| consumers.eat_line(x));

    // if res.is_ok() {
    // let res = res.unwrap();
    // let data = json!(consumers.to_json());
    let data = json!(consumers.get_data());
    return Ok(Response::new(Body::from(data.to_string())));
    // } else {
    //     return Ok(Response::new(Body::from(format!(
    //         "Error: {}",
    //         res.err().unwrap()
    //     ))));
    // }
    // let path = request.uri().path();
    // match serve_file(&path).await {
    //     Ok(file_content) => Ok(Response::new(Body::from(file_content))),
    //     Ok(_) => Ok(Response::new(Body::from(format!("Path: {}", path)))),
    //     Err(_) => Ok(Response::builder().status(404).body(Body::empty()).unwrap()),
    // }
    // Ok(Response::new(Body::from(format!("Path: {}", "123"))))
}

// fn router_service() -> Result<RouterService, std::io::Error> {
//     let router = RouterBuilder::new()
//         .add(Route::get("/hello").using(request_handler))
//         .add(Route::from(Method::PATCH, "/asd").using(request_handler))
//         .build();

//     Ok(RouterService::new(router))
// }

#[tokio::main]
pub async fn start(port: u16) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(hello)) });
    let addr = ([127, 0, 0, 1], port).into();
    let server = Server::bind(&addr).serve(make_svc);
    println!("Listening on http://{}", addr);
    server.await?;
    Ok(())
}
