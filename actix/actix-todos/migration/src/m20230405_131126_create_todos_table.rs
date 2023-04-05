use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Todo {
    Table,
    Id,
    Name,
    Status,
    UserId,
}


#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Todo::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Todo::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Todo::Name).string().not_null())
                    .col(ColumnDef::new(Todo::Status).boolean().not_null())
                    .col(ColumnDef::new(Todo::UserId).integer().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Todo::Table).to_owned())
            .await
    }
}

