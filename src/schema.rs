table! {
    category (id) {
        id -> Integer,
        descript -> Varchar,
        icon -> Varchar,
        url -> Varchar,
        label -> Varchar,
        name -> Varchar,
        showIndex -> Bool,
        isDelete -> Bool,
        sort -> Integer,
        pid -> Nullable<Integer>,
    }
}

table! {
    post (id) {
        id -> Integer,
        name -> Varchar,
        postdata -> Varchar,
        user -> Integer,
        service -> Integer,
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
    user (id) {
        id -> Integer,
        avater -> Varchar,
        account -> Varchar,
        password -> Varchar,
        salt -> Varchar,
        name -> Varchar,
        sex -> Integer,
        email -> Varchar,
        phone -> Varchar,
        roleid -> Varchar,
        deptid -> Varchar,
        status -> Integer,
        version -> Integer,
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
    category,
    post,
    service,
    user,
    users,
);
