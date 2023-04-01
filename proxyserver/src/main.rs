use std::net::{SocketAddr, ToSocketAddrs};
use hyper::{Body, Client, Request, Response, Server};
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use tokio::net::TcpListener;
use tokio::sync::oneshot;

async fn forward_request(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    // Replace the following URL with the destination URL you want to proxy to
    let url = "http://example.com".parse().unwrap();
    let mut dest_req = Request::new(req.body());
    *dest_req.uri_mut() = url;

    // Add headers from the original request to the destination request
    for (name, value) in req.headers() {
        dest_req.headers_mut().insert(name, value.clone());
    }

    // Send the destination request and return the response
    let client = Client::new();
    client.request(dest_req).await
}

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    forward_request(req).await
}

async fn run_server(addr: SocketAddr, shutdown: oneshot::Receiver<()>) {
    let make_svc = make_service_fn(|socket: &AddrStream| async move {
        Ok::<_, hyper::Error>(service_fn(handle_request))
    });
    let server = Server::bind(&addr).serve(make_svc);
    let graceful = server.with_graceful_shutdown(async {
        shutdown.await.ok();
    });
    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }
}

#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:8080".to_socket_addrs().unwrap().next().unwrap();
    let (shutdown_tx, shutdown_rx) = oneshot::channel();
    let server = run_server(addr, shutdown_rx);
    println!("Listening on {}", addr);
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}