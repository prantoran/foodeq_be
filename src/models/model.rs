//! Simplistic model layer
//! (with mock-store layer)

use crate::{ctx::Ctx, error::{Error, Result}};
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

// -- Model Controller

#[derive(Clone)] // Clones the Arc, not the vector
pub struct ModelController {
    // FIXME: Use a real database connection or ORM in production.
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>, 
}

// Constructor
impl ModelController {
    // Control the signature of the constructor early on,
    // so that we can swap the implementation later.
    pub async fn new() -> Result<Self> {
        Ok(Self {
            tickets_store: Arc::default(),
        })
    }
}

// CRUD Implementation
impl ModelController {
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

// End: -- Model Controller

