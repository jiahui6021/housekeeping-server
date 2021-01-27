use crate::schema::post;
#[derive(Insertable)]
#[table_name = "post"]
pub struct Post {
    pub id: i32,
    pub username: String,
    pub postdata: String,
}