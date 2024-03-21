use std::convert::Infallible;

use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use serde_json::json;

use crate::{conf, Consumers, FileFinder, LogFile, Parser, ResourceConsumer, VisitorsConsumer, StatusConsumer};

fn feed_consumers(consumers: &mut Consumers) {
    let conf = conf::get().lock().unwrap();
    let paths = [conf.base_dir.clone()];
    let parser = Parser::new(conf.format.as_str()).unwrap();

    FileFinder::new(&paths, &parser)
        .dated_files()
        .unwrap()
        .into_iter()
        .map(|x| x.1)
        .flat_map(|x| LogFile::new(&x, &parser).into_iter())
        .for_each(|x| consumers.eat_line(x));
}

async fn hello(request: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut consumers = Consumers::new();
    consumers.add_consumer(Box::new(ResourceConsumer::default()));
    consumers.add_consumer(Box::new(VisitorsConsumer::default()));
    consumers.add_consumer(Box::new(StatusConsumer::default()));

    feed_consumers(&mut consumers);
    let data = json!(consumers.get_data());
    Ok(Response::new(Body::from(data.to_string())))
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
