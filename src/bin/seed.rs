extern crate chrono;
extern crate diesel;
#[macro_use] 
extern crate fake;
extern crate rand;
extern crate reorg;

use chrono::NaiveDate;
use diesel::prelude::*;
use rand::Rng;
use reorg::*;
use reorg::models::*;

fn main() {
    use reorg::schema::conferences::dsl::*;
    use reorg::schema::reviews::dsl::*;
    use reorg::schema::reviewers::dsl::*;
    use reorg::schema::submissions::dsl::*;
    use reorg::schema::users::dsl::*;

    let connection = create_db_pool().get().unwrap();

    // Clean out old data
    diesel::delete(conferences).execute(&*connection).expect("Error deleteing conferences");
    diesel::delete(reviews).execute(&*connection).expect("Error deleteing reviews");
    diesel::delete(reviewers).execute(&*connection).expect("Error deleteing reviewers");
    diesel::delete(submissions).execute(&*connection).expect("Error deleteing submissions");
    diesel::delete(users).execute(&*connection).expect("Error deleteing users");

    // Reset the primary keys
    connection.execute("ALTER SEQUENCE conferences_id_seq RESTART WITH 1").expect("Error resetting conference id");
    connection.execute("ALTER SEQUENCE reviews_id_seq RESTART WITH 1").expect("Error resetting reviews id");
    connection.execute("ALTER SEQUENCE reviewers_id_seq RESTART WITH 1").expect("Error resetting reviewers id");
    connection.execute("ALTER SEQUENCE submissions_id_seq RESTART WITH 1").expect("Error resetting submissions id");
    connection.execute("ALTER SEQUENCE users_id_seq RESTART WITH 1").expect("Error resetting users id");
    
    fn generate_conference() -> NewConference {
        NewConference {
            title: fake!(Company.name),
            start_date: NaiveDate::from_ymd(2017, 11, 12).and_hms(17, 33, 44),   
            end_date: NaiveDate::from_ymd(2017, 11, 12).and_hms(17, 33, 44),
            venue: fake!(Company.name),
            address: fake!(Address.street_address),
            city: fake!(Address.city),
            postcode: fake!(Address.postcode),
            country: fake!(Address.state).to_string(),
            cfp: fake!(Lorem.paragraph(7, 3)),
        }
    }

    fn generate_submission(conf: i32) -> NewSubmission {
        let mut rng = rand::thread_rng();

        NewSubmission {
            conference_id: conf,
            user_id: rng.gen_range(1, 11),
            title: fake!(Lorem.sentence(4, 6)),
            content: fake!(Lorem.paragraph(7, 3)),
        }
    }

    fn generate_user() -> NewUser {
        NewUser {
            first_name: fake!(Name.first_name).to_string(),
            last_name: fake!(Name.last_name).to_string(),
            email: fake!(Internet.free_email),
            password: fake!(Lorem.word).to_string(),
        }
    }

    fn generate_reviewer(conf: i32, rev: i32) -> NewReviewer {
        NewReviewer {
            conference_id: conf,
            user_id: rev,
        }
    }

    fn generate_review(conf: i32) -> NewReview {
        NewReview {
            reviewer_id: conf,
            submission_id: conf,
            private_comments: fake!(Lorem.paragraph(7, 3)),
            shared_comments: fake!(Lorem.paragraph(7, 3)),
        }
    }

    // Seed new data
    for _w in 0..10 {
        create_user(&connection, &generate_user());
    }
    for x in 1..11 {
        create_conference(&connection, &generate_conference());
        for _y in 0..5 {
            create_submission(&connection, &generate_submission(x));
        }
        for z in 1..11 {
            create_review(&*connection, &generate_review(x));
            create_reviewer(&*connection, &generate_reviewer(x, z));
        }
    }
}