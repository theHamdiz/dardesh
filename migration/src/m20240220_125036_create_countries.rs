use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Country::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Country::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Country::Flag).string().not_null())
                    .col(ColumnDef::new(Country::NameEn).string().not_null())
                    .col(ColumnDef::new(Country::NameAr).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Country::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub(crate) enum Country {
    Table,
    Id,
    NameEn,
    NameAr,
    Flag,
}
