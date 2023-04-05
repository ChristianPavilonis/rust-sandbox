pub use sea_orm_migration::prelude::*;

mod m20230402_214115_create_users_table;
mod m20230405_131126_create_todos_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230402_214115_create_users_table::Migration),
            Box::new(m20230405_131126_create_todos_table::Migration),
        ]
    }
}
