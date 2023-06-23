use std::time::SystemTime;

use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use super::schema::users;
use super::schema::users::dsl::users as all_users;
use super::schema::comments;


#[derive(Serialize, Queryable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub is_sub: bool,
    pub is_partner: bool,
    pub is_mod: bool,
    pub is_vip: bool,
    pub is_admin: bool,
    pub is_broadcaster: bool
}

#[derive(Deserialize)]
pub struct UserData {
    pub username: String,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub is_sub: bool,
    pub is_partner: bool,
    pub is_mod: bool,
    pub is_vip: bool,
    pub is_admin: bool,
    pub is_broadcaster: bool
}

impl User {
    pub fn insert_user(user: NewUser, conn: &PgConnection) -> Vec<User> {
        return diesel::insert_into(users::table)
        .values(&user)
        .get_results(conn)
        .expect("Error");
    }

    pub fn check_if_user_exists(username: &String, conn: &PgConnection) -> Vec<User> {
        return all_users
        .filter(users::username.eq(username))
        .load::<User>(conn)
        .expect("Error")
    }
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "comments"]
pub struct NewComment {
    pub user_id: i32,
    pub comment: String,
    pub created_at: SystemTime,
}

impl NewComment {
    pub fn insert_comment(comment: NewComment, conn: &PgConnection) -> bool {
        diesel::insert_into(comments::table)
            .values(comment)
            .execute(conn)
            .is_ok()
    }
}