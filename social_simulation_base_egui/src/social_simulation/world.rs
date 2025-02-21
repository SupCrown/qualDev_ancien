use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct World {
    pub width: u32,
    pub height: u32,
}
