use std::str::FromStr;

use super::recipient::Recipient;
use super::sender::{select_sender, Sender};
use crate::schema::{messages, users};
use diesel::{insert_into, prelude::*, update};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq, Serialize)]
#[diesel(belongs_to(Recipient))]
#[diesel(table_name = messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Message {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub status: String,
    pub sender_id: Uuid,
    pub recipient_id: Uuid,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = messages)]
pub struct NewMessage {
    pub title: String,
    pub body: String,
    pub status: String,
    pub sender_id: Uuid,
    pub recipient_id: Uuid,
}

pub fn select_all_messages(
    conn: &mut PgConnection,
    username: &str,
) -> diesel::QueryResult<Vec<(Message, Sender)>> {
    let recipient = users::table
        .filter(users::name.eq(username))
        .select(Recipient::as_select())
        .first(conn)?;

    let found_messages: Vec<Message> = Message::belonging_to(&recipient)
        .select(Message::as_select())
        .load(conn)?;

    found_messages
        .into_iter()
        .map(|message| select_sender(message.sender_id, conn).map(|sender| (message, sender)))
        .collect()
}

pub fn select_all_messages_with_status(
    conn: &mut PgConnection,
    username: &str,
    status: &str,
) -> diesel::QueryResult<Vec<(Message, Sender)>> {
    let recipient = users::table
        .filter(users::name.eq(username))
        .select(Recipient::as_select())
        .first(conn)?;

    let found_messages: Vec<Message> = Message::belonging_to(&recipient)
        .filter(messages::status.eq(status))
        .select(Message::as_select())
        .load(conn)?;

    found_messages
        .into_iter()
        .map(|message| select_sender(message.sender_id, conn).map(|sender| (message, sender)))
        .collect()
}

pub fn change_message_status(
    conn: &mut PgConnection,
    username: &str,
    message_uuid: &str,
    status: &str,
) -> diesel::QueryResult<()> {
    let message_uuid = Uuid::from_str(message_uuid).unwrap(); // TODO: Fix unwrap
    let user = users::table
        .filter(users::name.eq(username))
        .select(Recipient::as_select())
        .load(conn)?;

    let _: Message = Message::belonging_to(&user)
        .filter(messages::id.eq(message_uuid))
        .first(conn)?;

    update(messages::table)
        .filter(messages::id.eq(message_uuid))
        .set(messages::status.eq(status))
        .execute(conn)?;

    Ok(())
}

pub fn create_new_message(message: NewMessage, conn: &mut PgConnection) -> QueryResult<usize> {
    insert_into(messages::table).values(&message).execute(conn)
}
