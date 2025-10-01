use axum::extract::{FromRef, Path, State};
use axum::routing::{delete, post};
use axum::{Json, Router};

use crate::ctx::Ctx;
use crate::model::model::{ModelManager, Ticket, TicketForCreate};
use crate::error::Result;

#[derive(Clone, FromRef)]
struct AppState {
    mm: ModelManager,
}

pub fn routes(mm: ModelManager) -> Router {
    let app_state = AppState {mm};
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/tickets/{id}", delete(delete_ticket))
        .with_state(app_state)
}

// REST Handlers for Ticket
async fn create_ticket(
    State(mm): State<ModelManager>,
    ctx: Ctx,
    Json(ticket_fc): Json<TicketForCreate>,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - create_ticket", "HANDLER");

    let ticket = mm.create_ticket(ctx, ticket_fc).await?;
    
    Ok(Json(ticket))
}

async fn list_tickets(
    State(mm): State<ModelManager>,
    ctx: Ctx,
) -> Result<Json<Vec<Ticket>>> {
    println!("->> {:<12} - list_tickets", "HANDLER");

    let tickets = mm.list_tickets(ctx).await?;
    
    Ok(Json(tickets))
}

async fn delete_ticket(
    State(mm): State<ModelManager>,
    ctx: Ctx,
    Path(id): Path<u64>,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - delete_ticket", "HANDLER");

    let ticket = mm.delete_ticket(ctx, id).await?;
    
    Ok(Json(ticket))
}

// END -- REST Handlers for Ticket