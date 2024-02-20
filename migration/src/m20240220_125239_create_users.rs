use sea_orm_migration::prelude::*;
use crate::m20240220_125036_create_countries::Country;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Username).string().not_null())
                    .col(ColumnDef::new(User::Age).integer().not_null())
                    .col(ColumnDef::new(User::Gender).string().not_null())
                    .col(ColumnDef::new(User::CountryId).string().null())
                    .col(ColumnDef::new(User::Avatar).string().null())
                    .col(ColumnDef::new(User::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(User::UpdatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_country")
                            .from(User::Table, User::CountryId)
                            .to(Country::Table, Country::Id),)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub(crate) enum User {
    Table,
    Id,
    Username,
    Age,
    Gender,
    CountryId,
    Avatar,
    CreatedAt,
    UpdatedAt,
}
