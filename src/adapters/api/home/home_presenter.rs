use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct HomePresenter {
    pub id: Uuid,
    pub name: String,
}
