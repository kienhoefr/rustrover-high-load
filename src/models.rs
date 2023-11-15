use diesel::{Queryable, Selectable};

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::todos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub done: bool,
}
