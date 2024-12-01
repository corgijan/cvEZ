use crate::posts::dsl::posts;
use diesel::prelude::*;
use crate::schema::posts as other_posts;
use crate::schema::posts::published;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}


#[derive(Insertable)]
#[diesel(table_name = other_posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub(crate) published: bool,
}