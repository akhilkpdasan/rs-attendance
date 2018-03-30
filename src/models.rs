use super::schema::students;

#[derive(Insertable, Queryable, PartialEq, Debug, Serialize, Deserialize)]
#[table_name = "students"]
pub struct Student {
    pub id: String,
    pub name: String,
    pub roll_no: i32,
    pub attendance: f32,
}

#[derive(Serialize)]
pub struct Students {
    students: Vec<Student>,
}
