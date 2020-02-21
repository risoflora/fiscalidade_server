table! {
    fiscalidade_taxpayers_services_view (taxpayer_id, service_id) {
        id -> Int8,
        taxpayer_id -> Int8,
        taxpayer_name -> Varchar,
        service_id -> Int8,
        service_description -> Varchar,
        allowed_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
    }
}
