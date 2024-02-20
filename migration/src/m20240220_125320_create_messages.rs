use sea_orm_migration::prelude::*;
use crate::m20240220_124003_create_rooms::Room;
use crate::m20240220_125239_create_users::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Message::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Message::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Message::Content).string().not_null())
                    .col(ColumnDef::new(Message::SenderId).uuid().not_null())
                    .col(ColumnDef::new(Message::ReceiverId).uuid().not_null())
                    .col(ColumnDef::new(Message::ReplyToId).uuid().null())
                    .col(ColumnDef::new(Message::RoomId).uuid().not_null())
                    .col(ColumnDef::new(Message::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Message::UpdatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Message::SeenAt).timestamp().null())
                    .col(ColumnDef::new(Message::IsSeen).boolean().not_null().default(false))
                    .foreign_key(ForeignKey::create().from(Message::Table, Message::SenderId).to(User::Table, User::Id))
                    .foreign_key(ForeignKey::create().from(Message::Table, Message::RoomId).to(Room::Table, Room::Id))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Message::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Message {
    Table,
    Id,
    SenderId,
    ReceiverId,
    ReplyToId,
    RoomId,
    Content,
    CreatedAt,
    UpdatedAt,
    SeenAt,
    IsSeen,
}