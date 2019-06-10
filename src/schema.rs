table! {
    conferences (id) {
        id -> Int4,
        title -> Varchar,
        start_date -> Timestamp,
        end_date -> Timestamp,
        venue -> Varchar,
        address -> Varchar,
        city -> Varchar,
        postcode -> Varchar,
        country -> Varchar,
        cfp -> Varchar,
    }
}
