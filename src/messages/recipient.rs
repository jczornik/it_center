use crate::schema::users;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = users)]
pub struct Recipient {
    pub id: Uuid,
    pub name: String,
}

pub fn get_recipient_by_name(username: &str, conn: &mut PgConnection) -> QueryResult<Recipient> {
    users::table
        .select(Recipient::as_select())
        .filter(users::name.eq(username))
        .get_result(conn)
}
