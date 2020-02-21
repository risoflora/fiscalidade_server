table! {
    fiscalidade_caches (id) {
        id -> Int8,
        key -> Varchar,
        value -> Bytea,
        created_at -> Timestamp,
    }
}

table! {
    fiscalidade_services (id) {
        id -> Int8,
        description -> Varchar,
        slug -> Varchar,
        active -> Bool,
        created_at -> Timestamp,
    }
}

table! {
    fiscalidade_taxpayers (id) {
        id -> Int8,
        name -> Varchar,
        business_name -> Varchar,
        registry -> Varchar,
        email -> Varchar,
        certificate -> Text,
        certificate_password -> Varchar,
        token -> Varchar,
        manager -> Bool,
        active -> Bool,
        created_at -> Timestamp,
    }
}

table! {
    fiscalidade_taxpayers_services (id) {
        id -> Int8,
        taxpayer_id -> Int8,
        service_id -> Int8,
        allowed_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
    }
}

joinable!(fiscalidade_taxpayers_services -> fiscalidade_services (service_id));
joinable!(fiscalidade_taxpayers_services -> fiscalidade_taxpayers (taxpayer_id));

allow_tables_to_appear_in_same_query!(
    fiscalidade_caches,
    fiscalidade_services,
    fiscalidade_taxpayers,
    fiscalidade_taxpayers_services,
);
