use crate::schema::post;
#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub username: String,
    pub postdata: String,
}

#[derive(Insertable)]
#[table_name = "post"]
pub struct NewPost {
    pub username: String,
    pub postdata: String,
}