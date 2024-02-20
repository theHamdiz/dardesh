pub use sea_orm_migration::prelude::*;
mod m20240220_124003_create_rooms;
mod m20240220_125036_create_countries;
mod m20240220_125239_create_users;
mod m20240220_125320_create_messages;
mod m20240220_130646_create_user_room;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240220_124003_create_rooms::Migration),
            Box::new(m20240220_125036_create_countries::Migration),
            Box::new(m20240220_125239_create_users::Migration),
            Box::new(m20240220_125320_create_messages::Migration),
            Box::new(m20240220_130646_create_user_room::Migration),
        ]
    }
}
