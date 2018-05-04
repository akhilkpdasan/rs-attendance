table! {
    students (id) {
        id -> Text,
        name -> Text,
        roll_no -> Int4,
        attendance -> Float4,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Text,
        username -> Text,
        password -> Text,
    }
}

allow_tables_to_appear_in_same_query!(students, users,);
