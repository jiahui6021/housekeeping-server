table! {
    pos (country, province, street) {
        country -> Integer,
        province -> Integer,
        street -> Integer,
        service -> Integer,
    }
}

table! {
    post (id) {
        id -> Integer,
        username -> Varchar,
        postdata -> Varchar,
    }
}

table! {
    service (id) {
        id -> Integer,
        province -> Integer,
        city -> Integer,
        street -> Integer,
        name -> Varchar,
        price -> Integer,
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
    pos,
    post,
    service,
    users,
);
