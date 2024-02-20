use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Room::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Room::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Room::NameAr).string().not_null())
                    .col(ColumnDef::new(Room::NameEn).string().not_null())
                    .col(ColumnDef::new(Room::Avatar).string().not_null())
                    .col(ColumnDef::new(Room::RoomType).string().not_null())
                    .col(ColumnDef::new(Room::MaxMembers).tiny_unsigned().not_null())
                    .col(ColumnDef::new(Room::Password).string().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Room::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub(crate) enum Room {
    Table,
    Id,
    NameEn,
    NameAr,
    Avatar,
    RoomType,
    MaxMembers,
    Password,
}