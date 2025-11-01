//! Model layer
//! (with mock-store layer)

use crate::{ctx::Ctx, model::{Result, Error}, model::store::{Db, new_db_pool}};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// -- Ticket Types

#[derive(Clone, Debug, Serialize)] // Clone: need to send copy back to the client
pub struct Ticket {
    pub id: u64,
    pub cid: u64, // creator user id
    pub title: String,
}

#[derive(Deserialize)]
pub struct TicketForCreate {
    pub title: String,
}

// End: --Ticket Types

// -- Model Manager

#[derive(Clone)] // Clones the Arc, not the vector
pub struct ModelManager {
    // FIXME: Use a real database connection or ORM in production.
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>, 
    db: Db,
    // we want to expose the db pool only to the model layer, done using pub(in crate::model) in impl ...
}

// Constructor
impl ModelManager {
    // Control the signature of the constructor early on,
    // so that we can swap the implementation later.
    // We also want to have he new() accessible to modules such as main.rs
    // new() is accessible to all the code base that has access to the ModelManager.
    pub async fn new() -> std::result::Result<Self, Error> { // Constructor
        let db = new_db_pool().await?; // new_db_pool can return a store error, but the Result is from the model layer, so we need to implement a variant in the model::error to map it.
        Ok(ModelManager{
            tickets_store: Arc::default(),
            db: db,
        })
    }

    // In crate model, restrict only to the modules that are below the model layer
    // Returns the sqlx db pool reference.
    // Only for the model layer
    pub(in crate::model) fn db(&self) -> &Db {
        &self.db
    }
}

// CRUD Implementation
impl ModelManager {
    pub async fn create_ticket(
        &self,
        ctx: Ctx,
        ticket_fc: TicketForCreate
    ) -> Result<Ticket> {
        let mut store= self.tickets_store.lock().unwrap();
        
        let id = store.len() as u64;
        let ticket = Ticket {
            id,
            cid: ctx.user_id(),
            title: ticket_fc.title,
        };

        store.push(Some(ticket.clone()));

        Ok(ticket)
        // todo!();
    }

    pub async fn list_tickets(
        &self,
        _ctx: Ctx
    ) -> Result<Vec<Ticket>> {
        let store = self.tickets_store.lock().unwrap();
        
        // Filter out None values and collect the Some values
        let tickets: Vec<Ticket> = store.iter()
            .filter_map(|ticket| ticket.clone())
            .collect();

        Ok(tickets)
    }

    pub async fn delete_ticket(
        &self,
        _ctx: Ctx,
        id: u64
    ) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();
        
        let ticket = store.get_mut(id as usize)
            .and_then(|t| t.take());

        ticket.ok_or(Error::TicketDeleteFailIdNotFound { id })
    }
}

// End: -- Model Manager




 