// @generated automatically by Diesel CLI.

diesel::table! {
    babies (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    dreams (id) {
        id -> Integer,
        baby_id -> Integer,
        from_date -> Timestamp,
        from_time -> Timestamp,
        to_date -> Nullable<Timestamp>,
        to_time -> Nullable<Timestamp>,
    }
}

diesel::table! {
    meals (id) {
        id -> Integer,
        baby_id -> Integer,
        date -> Timestamp,
        quantity -> Nullable<Integer>,
        elapsed -> Nullable<Integer>,
    }
}

diesel::table! {
    roles (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        password -> Text,
        name -> Nullable<Text>,
        surname -> Nullable<Text>,
        email -> Text,
        active -> Bool,
    }
}

diesel::table! {
    users_babies (id) {
        id -> Integer,
        baby_id -> Integer,
        user_id -> Integer,
    }
}

diesel::table! {
    users_roles (id) {
        id -> Integer,
        rol_id -> Integer,
        user_id -> Integer,
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
