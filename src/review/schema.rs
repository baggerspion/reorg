table! {
    reviews (id) {
        id -> Nullable<Int4>,
        reviewer_id -> Int4,
        submission_id -> Int4,
        private_comments -> Varchar,
        shared_comments -> Varchar,
        score -> Int4,
    }
}