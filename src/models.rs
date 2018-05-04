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

#[derive(Insertable, Queryable, PartialEq, Debug, Serialize, Deserialize)]
#[table_name = "users"]
pub struct Users {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub password: String,
}

//TODO move this to appropriate file
#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct UserRegister {
    pub username: String,
    pub password: String,
    pub email: String,
}
