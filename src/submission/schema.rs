table! {
    submissions (id) {
        id -> Nullable<Int4>,
        conference_id -> Int4,
        user_id -> Int4,
        title -> Varchar,
        content -> Varchar,
    }
}