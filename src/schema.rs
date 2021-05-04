table! {
    addr (id) {
        id -> Integer,
        idUser -> Integer,
        addressDetail -> Varchar,
        areaCode -> Varchar,
        city -> Varchar,
        district -> Varchar,
        isDefault -> Bool,
        name -> Varchar,
        postCode -> Varchar,
        province -> Varchar,
        tel -> Varchar,
    }
}

table! {
    cart (id) {
        id -> Integer,
        count -> Integer,
        idGoods -> Integer,
        idSku -> Nullable<Integer>,
        user_id -> Integer,
    }
}

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
    goods (id) {
        id -> Integer,
        name -> Varchar,
        descript -> Varchar,
        gallery -> Varchar,
        pic -> Varchar,
        detail -> Varchar,
        price -> Integer,
        stock -> Integer,
        idCategory -> Integer,
        isOnSale -> Bool,
        isHot -> Bool,
        isNew -> Bool,
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
    shop_user (id) {
        id -> Integer,
        mobile -> Varchar,
        password -> Varchar,
        nickName -> Varchar,
        avatar -> Varchar,
        gender -> Varchar,
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
    addr,
    cart,
    category,
    goods,
    post,
    service,
    shop_user,
    user,
    users,
);
