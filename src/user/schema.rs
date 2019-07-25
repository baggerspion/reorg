table! {
    users (id) {
        id -> Nullable<Int4>,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        twitter -> Nullable<Varchar>,
        website -> Nullable<Varchar>,
        roles -> Nullable<Array<Varchar>>,
    }
}