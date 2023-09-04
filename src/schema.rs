// @generated automatically by Diesel CLI.

diesel::table! {
    babies (id) {
        id -> Int4,
        unique_id -> Uuid,
        name -> Varchar,
        birthdate -> Date,
        belongs_to -> Int4,
        added_on -> Timestamp,
    }
}

diesel::table! {
    dreams (id) {
        id -> Int4,
        baby_id -> Int4,
        from_date -> Timestamp,
        to_date -> Nullable<Timestamp>,
    }
}

diesel::table! {
    meals (id) {
        id -> Int4,
        baby_id -> Int4,
        date -> Timestamp,
        quantity -> Nullable<Int2>,
        to_time -> Nullable<Timestamp>,
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
        email -> Nullable<Varchar>,
        active -> Bool,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
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

diesel::table! {
    weights (id) {
        id -> Int4,
        baby_id -> Int4,
        date -> Date,
        value -> Float4,
    }
}

diesel::joinable!(babies -> users (belongs_to));
diesel::joinable!(dreams -> babies (baby_id));
diesel::joinable!(meals -> babies (baby_id));
diesel::joinable!(users_babies -> babies (baby_id));
diesel::joinable!(users_babies -> users (user_id));
diesel::joinable!(users_roles -> roles (rol_id));
diesel::joinable!(users_roles -> users (user_id));
diesel::joinable!(weights -> babies (baby_id));

diesel::allow_tables_to_appear_in_same_query!(
    babies,
    dreams,
    meals,
    roles,
    users,
    users_babies,
    users_roles,
    weights,
);
