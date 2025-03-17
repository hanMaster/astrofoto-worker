use crate::stuff::routes::get_router;
use crate::stuff::state::AppState;
pub use errors::Result;
use std::net::SocketAddr;
use tokio::net::TcpListener;

mod errors;
mod stuff;

#[tokio::main]
async fn main() -> Result<()> {
    let state = AppState::new()?;
    let addr = SocketAddr::from(([0, 0, 0, 0], state.port));
    let listener = TcpListener::bind(addr).await?;
    let app = get_router(state);
    axum::serve(listener, app).await?;
    Ok(())
}







