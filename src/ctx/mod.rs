#[derive(Clone, Debug)]
pub struct Ctx {
    user_id: u64,
}

// Constructor.
impl Ctx {

    // Used by system, for tests, etc
    pub fn root_ctx() -> Self {
        Self { user_id: 0 } // root user
    }

    pub fn new(user_id: u64) -> Self {
        Self { user_id }
    }
}

// Property Accessors.
impl Ctx {
    pub fn user_id(&self) -> u64 {
        self.user_id
    }
}