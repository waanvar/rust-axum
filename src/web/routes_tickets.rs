
use axum::extract::{Path, State};
use axum::{
    routing::{delete, post},
    Router,
    Json
};

use crate::ctx::Ctx;
use crate::model::{ModelController , Ticket , TicketForCreate};
use crate::Result;

pub fn routes(mc:ModelController) -> Router {
    Router::new()
        .route("/tickets",post(create_ticket).get(list_tickets))
        .route("/tickets/{id}",delete(delete_tickets))
        .with_state(mc)
}


async fn create_ticket (
    mc : State<ModelController>,
    ctx : Ctx,
    Json(ticket_fc) : Json<TicketForCreate>,
) -> Result<Json<Ticket>>{
    println!("-->> {:<12} - create_ticket", "HANDLER");

    let ticket = mc.create_ticket(ctx,ticket_fc).await?;

    Ok(Json(ticket))
}


async fn list_tickets (
    State(mc) : State<ModelController>,
    ctx : Ctx,
) -> Result<Json<Vec<Ticket>>>{
    println!("-->> {:<12} - list_tickets", "HANDLER");

    let ticket = mc.list_tickets(ctx).await?;

    Ok(Json(ticket))
}

async fn delete_tickets (
    State(mc) : State<ModelController>,
    ctx : Ctx,
    Path(id) : Path<u64>,
) -> Result<Json<Ticket>>{
    println!("-->> {:<12} - list_tickets", "HANDLER");

    let ticket = mc.delete_ticket(ctx,id).await?;

    Ok(Json(ticket))
}


