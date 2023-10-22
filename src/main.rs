use hyper::service::{make_service_fn, service_fn};
use hyper::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = (([127, 0, 0, 1], 3000)).into();
    let page_path = "./pages";

    // for every connection, we must make a `Service` to handle all
    // incomming HTTP request on said connection.
    let service = make_service_fn(|_| async {
        // This is the `Service` that will handle the connection.
        // `service_fn` is a helper to convert a function that
        // returns a Response into a `Service`.
        // Ok::<_, hyper::Error>(service_fn(|req| wc_note::handle(req, page_path)))

        // wc_node::service()
        Ok::<_, hyper::Error>(service_fn(|req| wc_note::service(req, page_path)))
    });

    let server = Server::bind(&addr).serve(service);

    println!("Listening on http:://{}", addr);

    server.await?;

    Ok(())
}

// page data update
// Web -- Server
// file data to client
// wasm make html form data
