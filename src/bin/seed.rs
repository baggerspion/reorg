extern crate chrono;
extern crate diesel;
#[macro_use] 
extern crate fake;
extern crate rand;
extern crate reorg;
extern crate serde;

use chrono::NaiveDate;
use diesel::prelude::*;
use rand::Rng;
use rand::rngs::ThreadRng;
use reorg::*;
use reorg::models::*;

fn main() {
    // Clean out old test data
    let connection = create_db_pool().get().unwrap();
    let mut rng = rand::thread_rng();
    
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

    fn generate_submission(rng: &mut ThreadRng) -> NewSubmission {
        NewSubmission {
            conference_id: rng.gen_range(1, 11),
            user_id: rng.gen_range(1, 21),
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

    fn generate_reviewer(rng: &mut ThreadRng) -> NewReviewer {
        NewReviewer {
            conference_id: rng.gen_range(1, 11),
            user_id: rng.gen_range(1, 21),
        }
    }

    fn generate_review(rng: &mut ThreadRng) -> NewReview {
        NewReview {
            reviewer_id: rng.gen_range(1, 11),
            submission_id: rng.gen_range(1,101),
            private_comments: fake!(Lorem.paragraph(7, 3)),
            shared_comments: fake!(Lorem.paragraph(7, 3)),
        }
    }

    for _x in 0..20 {
        create_user(&connection, &generate_user());
    }
    for _y in 0..10 {
        create_conference(&connection, &generate_conference());
    }
    for _z in 0..100 {
        create_submission(&connection, &generate_submission(&mut rng));
    }
    for _a in 0..10 {
        create_reviewer(&connection, &generate_reviewer(&mut rng));
    }
    for _b in 1..40 {
        create_review(&connection, &generate_review(&mut rng));
    }


    // Read some reviews
    use reorg::schema::submissions::dsl::*;
    let first_submission = submissions.limit(1)
        .load::<Submission>(&*connection)
        .expect("Error loading posts");
    let reviews = read_reviews(&connection, &first_submission[0]);
    for review in reviews {
        println!{"{}", review.shared_comments};
    }
}