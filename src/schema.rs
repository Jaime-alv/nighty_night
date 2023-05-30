// @generated automatically by Diesel CLI.

diesel::table! {
    babies (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    dreams (id) {
        id -> Int4,
        baby_id -> Int4,
        from_date -> Timestamp,
        from_time -> Timestamp,
        to_date -> Nullable<Timestamp>,
        to_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    meals (id) {
        id -> Int4,
        baby_id -> Int4,
        date -> Timestamp,
        quantity -> Nullable<Int2>,
        elapsed -> Nullable<Int2>,
    }
}

diesel::table! {
    roles (id) {
        id -> Int2,
        name -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        name -> Nullable<Varchar>,
        surname -> Nullable<Varchar>,
        email -> Varchar,
        active -> Bool,
    }
}

diesel::table! {
    users_babies (id) {
        id -> Int4,
        baby_id -> Int4,
        user_id -> Int4,
    }
}

diesel::table! {
    users_roles (id) {
        id -> Int4,
        rol_id -> Int2,
        user_id -> Int4,
    }
}

diesel::joinable!(dreams -> babies (baby_id));
diesel::joinable!(meals -> babies (baby_id));
diesel::joinable!(users_babies -> babies (baby_id));
diesel::joinable!(users_babies -> users (user_id));
diesel::joinable!(users_roles -> roles (rol_id));
diesel::joinable!(users_roles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    babies,
    dreams,
    meals,
    roles,
    users,
    users_babies,
    users_roles,
);
