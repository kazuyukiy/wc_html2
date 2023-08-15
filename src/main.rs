use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use std::convert::Infallible;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init();

    // for every connection, we must make a `Service` to handle all
    // incomming HTTP request on said connection.
    let make_svc = make_service_fn(|_conn| {
        // This is the `Service` that will handle the connection.
        // `service_fn` is a helper to convert a function that
        // returns a Response into a `Service`.
        // async { Ok::<_, Infallible>(service_fn(wc_note::hello_world)) }
        async { Ok::<_, Infallible>(service_fn(wc_note::handle)) }
    });

    let addr = SocketAddr::from(([127, 0, 0, 1], 7878));

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http:://{}", addr);

    server.await?;

    Ok(())
}

// page data update
// Web -- Server
// file data to client
// wasm make html form data
