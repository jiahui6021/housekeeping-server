table! {
    post (id) {
        id -> Integer,
        username -> Varchar,
        postdata -> Varchar,
    }
}

table! {
    users (id) {
        id -> Integer,
        username -> Varchar,
        password -> Varchar,
        email -> Varchar,
        age -> Nullable<Integer>,
        sex -> Nullable<Bool>,
    }
}

allow_tables_to_appear_in_same_query!(
    post,
    users,
);
