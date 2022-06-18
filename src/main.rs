use breadx::{prelude::*, AsyncDisplayConnection};
use std::error::Error;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut conn = AsyncDisplayConnection::create_async(None, None).await?;
    let root = conn.default_screen().root;

    let mut qpr = breadx::auto::xproto::QueryPointerRequest::default();
    qpr.window = root;

    let req_cookie = conn.send_request_async(qpr).await?;
    let reply = conn.resolve_request_async(req_cookie).await?;
    println!("Pointer is at x:{}, y:{}", reply.root_x, reply.root_y);
    Ok(())
}
