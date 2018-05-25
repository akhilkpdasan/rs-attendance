use super::schema::students;
use super::schema::users;

#[derive(Insertable, Queryable, PartialEq, Debug, Serialize, Deserialize)]
#[table_name = "students"]
pub struct Student {
    pub id: String,
    pub name: String,
    pub roll_no: i32,
    pub attendance: f32,
}

#[derive(Queryable, PartialEq, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub email: String,
}

//TODO move this to appriopriate place
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub username: String,
}
