use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct DardeshError{
    pub message: String
}

impl From<uuid::Error> for DardeshError{
    fn from(err: uuid::Error) -> Self {
        DardeshError{
            message: err.to_string()
        }
    }
}

impl From<sea_orm::DbErr> for DardeshError{
    fn from(err: sea_orm::DbErr) -> Self {
        DardeshError{
            message: err.to_string()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Ord, PartialOrd, PartialEq, Eq)]
pub struct UserRoom {
    user_id: Uuid,
    room_id: Uuid,
}


#[derive(Serialize, Deserialize, Debug, Ord, PartialOrd, PartialEq, Eq)]
pub enum RoomType {
    Global,
    Public,
    Private,
    Protected,
}

#[derive(Serialize, Deserialize, Debug, Ord, PartialOrd, PartialEq, Eq)]

pub enum Gender{
    Male,
    Female,
    Other,
}

#[derive(Serialize, Deserialize, Debug, Ord, PartialOrd, PartialEq, Eq)]
pub struct Room {
    id: Uuid,
    name_en: String,
    name_ar: String,
    avatar: String,
    max_members: u8,
    room_type: RoomType,
    password: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Ord, PartialOrd, PartialEq, Eq)]
pub struct DbUser {
    id: Uuid,
    username: String,
    age: u8,
    gender: Gender,
    country_id: Option<Uuid>,
    avatar: Option<String>,
    created_at: i64,
    updated_at: i64,
}

#[derive(Serialize, Deserialize, Debug, Ord, PartialOrd, PartialEq, Eq)]
pub struct DbMessage {
    id: Uuid,
    content: String,
    sender_id: String,
    room_id: String,
    receiver_id: String,
    reply_to_id: Option<String>,
    created_at: i64,
    updated_at: i64,
    seen_at: i64,
    is_seen: bool,
}

#[derive(Serialize, Deserialize, Debug, Ord, PartialOrd, PartialEq, Eq)]
pub struct Country{
    id: Uuid,
    name_en: String,
    name_ar: String,
    flag: String,
}

#[derive(Serialize, Deserialize, Debug, Ord, PartialOrd, PartialEq, Eq)]
pub struct Interest{
    id: Uuid,
    name_en: String,
    name_ar: String,
}
