table! {
    users (id) {
        id -> Nullable<Int4>,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        roles -> Nullable<Array<Varchar>>,
    }
}