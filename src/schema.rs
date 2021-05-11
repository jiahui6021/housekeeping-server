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
    article (id) {
        id -> Integer,
        author -> Varchar,
        content -> Varchar,
        idChannel -> Varchar,
        img -> Varchar,
        title -> Varchar,
    }
}

table! {
    banner (id) {
        id -> Integer,
        idFile -> Varchar,
        page -> Varchar,
        param -> Varchar,
        title -> Varchar,
    }
}

table! {
    cart (id) {
        id -> Integer,
        count -> Integer,
        idGoods -> Integer,
        idSku -> Nullable<Integer>,
        user_id -> Integer,
        order_id -> Nullable<Integer>,
    }
}

table! {
    cat_banner (id) {
        id -> Integer,
        car_id -> Integer,
        banner_id -> Integer,
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
        banner_id -> Varchar,
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
    like (id) {
        id -> Integer,
        user_id -> Integer,
        goods_id -> Integer,
    }
}

table! {
    order (id) {
        id -> Integer,
        idAddress -> Integer,
        idUser -> Integer,
        payId -> Nullable<Integer>,
        payStatus -> Integer,
        status -> Integer,
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
    topic (id) {
        id -> Integer,
        id_article -> Integer,
        disabled -> Bool,
        idGoodsList -> Varchar,
        pv -> Integer,
        title -> Varchar,
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
    article,
    banner,
    cart,
    cat_banner,
    category,
    goods,
    like,
    order,
    post,
    service,
    shop_user,
    topic,
    user,
    users,
);
