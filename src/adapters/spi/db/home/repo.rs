use async_trait::async_trait;
use diesel::prelude::*;
use std::error::Error;
use std::sync::Arc;

use crate::adapters::spi::db::{
    db_connection::DbConnection,
    home::mapper::HomeDbMapper,
    home::models::{Home, NewHome},
};
use crate::application::{mappers::db_mapper::DbMapper, repositories::home_repo_abstract::HomeRepoAbstract};

use crate::domain::home_entity::HomeEntity;

pub struct HomeRepo {
    pub db: Arc<DbConnection>,
}

#[async_trait(?Send)]
impl HomeRepoAbstract for HomeRepo {
    async fn add(&self, name_v: &str) -> Result<HomeEntity, Box<dyn Error>> {
        use crate::adapters::spi::db::schema::homes::dsl::*;

        let mut conn = self.db.get_pool().get()?;
        let new_home = NewHome { name: name_v };

        let result = diesel::insert_into(homes).values(&new_home).get_result::<Home>(&mut conn);

        match result {
            Ok(model) => Ok(HomeDbMapper::to_entity(model)),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn delete(&self, name: &str) -> Result<HomeEntity, Box<dyn Error>> {
        use crate::adapters::spi::db::schema::homes::dsl::{homes, name as hname};
        let mut conn = self.db.get_pool().get()?;
        let result = diesel::delete(homes.filter(hname.eq(name))).get_result::<Home>(&mut conn);

        match result {
            Ok(model) => Ok(HomeDbMapper::to_entity(model)),
            Err(e) => Err(Box::new(e)),
        }
    }
    async fn list(&self) -> Result<Vec<HomeEntity>, Box<dyn Error>> {
        use crate::adapters::spi::db::schema::homes::dsl::*;
        let mut conn = self.db.get_pool().get()?;
        let result = homes.load::<Home>(&mut conn);

        match result {
            Ok(models) => Ok(models.into_iter().map(HomeDbMapper::to_entity).collect::<Vec<_>>()),
            Err(e) => Err(Box::new(e)),
        }
    }
}
