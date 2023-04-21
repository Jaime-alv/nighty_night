// @generated automatically by Diesel CLI.

diesel::table! {
    roles (rol_id) {
        rol_id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    task (task_id) {
        task_id -> Integer,
        user_id -> Integer,
        name -> Text,
        description -> Text,
        done -> Bool,
    }
}

diesel::table! {
    user_model (user_id) {
        user_id -> Integer,
        username -> Text,
        password -> Text,
        rol -> Integer,
        task -> Nullable<Integer>,
    }
}

diesel::joinable!(user_model -> roles (rol));

diesel::allow_tables_to_appear_in_same_query!(
    roles,
    task,
    user_model,
);
