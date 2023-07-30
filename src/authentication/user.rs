use actix_web_httpauth::extractors::basic::BasicAuth;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Selectable, Queryable, Debug, Serialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ExisitingUser {
    pub name: String,
    pub password: Option<String>,
}

pub fn find_user(credentails: BasicAuth, conn: &mut PgConnection) -> QueryResult<ExisitingUser> {
    use crate::schema::users::dsl::*;
    let user_password = credentails
        .password()
        .expect("Basic auth should contain not empty password");
    users
        .filter(
            name.eq(credentails.user_id())
                .and(password.eq(user_password)),
        )
        .select(ExisitingUser::as_select())
        .first(conn)
}
