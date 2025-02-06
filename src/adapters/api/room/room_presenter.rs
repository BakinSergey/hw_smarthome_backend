use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct RoomPresenter {
    pub id: Uuid,
    pub name: String,
    pub home_id: String,
}
