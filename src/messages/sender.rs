use crate::schema::users;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = users)]
pub struct Sender {
    pub id: Uuid,
    pub name: String,
}

pub fn select_sender(uuid: Uuid, conn: &mut PgConnection) -> QueryResult<Sender> {
    users::table
        .select(Sender::as_select())
        .filter(users::id.eq(uuid))
        .first(conn)
}

pub fn get_sender_by_name(username: &str, conn: &mut PgConnection) -> QueryResult<Sender> {
    users::table
        .select(Sender::as_select())
        .filter(users::name.eq(username))
        .get_result(conn)
}
