use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;

async fn hello_world(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match header(&req, "host") {
        Ok(v) => println!("host: {}", v),
        Err(_) => (),
    }

    if req.method() == hyper::Method::GET {
        println!("method: GET");
    }

    if req.method() == hyper::Method::POST {
        println!("method: POST");
    }

    Ok(Response::new("Hello, world".into()))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init();

    // for every connection, we must make a `Service` to handle all
    // incomming HTTP request on said connection.
    let make_svc = make_service_fn(|_conn| {
        // This is the `Service` that will handle the connection.
        // `service_fn` is a helper to convert a function that
        // returns a Response into a `Service`.
        async { Ok::<_, Infallible>(service_fn(hello_world)) }
    });

    let addr = SocketAddr::from(([127, 0, 0, 1], 7878));

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http:://{}", addr);

    server.await?;

    Ok(())
}

fn header<'a>(req: &'a Request<Body>, key: &str) -> Result<&'a str, ()> {
    let headers = req.headers();
    match headers.get(key) {
        Some(hv) => match hv.to_str() {
            Ok(v) => Ok(v),
            Err(_) => Err(()),
        },
        None => Err(()),
    }
}
