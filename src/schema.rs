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

table! {
    reviewers (id) {
        id -> Int4,
        conference_id -> Int4,
        user_id -> Int4,
    }
}

table! {
    reviews (id) {
        id -> Int4,
        reviewer_id -> Int4,
        submission_id -> Int4,
        private_comments -> Varchar,
        shared_comments -> Varchar,
    }
}

table! {
    submissions (id) {
        id -> Int4,
        conference_id -> Int4,
        user_id -> Int4,
        title -> Varchar,
        content -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        password -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    conferences,
    reviewers,
    reviews,
    submissions,
    users,
);
