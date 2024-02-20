use sea_orm_migration::prelude::*;
use crate::m20240220_125239_create_users::User;
use crate::m20240220_124003_create_rooms::Room;
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserRoom::Table)
                    .if_not_exists()
                    .primary_key(Index::create().col(UserRoom::UserId).col(UserRoom::RoomId))
                    .col(ColumnDef::new(UserRoom::UserId).string().not_null())
                    .col(ColumnDef::new(UserRoom::RoomId).string().not_null())
                    .foreign_key(ForeignKey::create().from(UserRoom::Table, UserRoom::UserId).to(User::Table, User::Id))
                    .foreign_key(ForeignKey::create().from(UserRoom::Table, UserRoom::RoomId).to(Room::Table, Room::Id))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserRoom::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserRoom {
    Table,
    UserId,
    RoomId,
}
