extern crate chrono;
extern crate diesel;
#[macro_use] 
extern crate fake;
extern crate rand;
extern crate reorg;

use chrono::NaiveDate;
use diesel::prelude::*;
use rand::Rng;
use reorg::data::create_db_pool;
use reorg::conference::model::Conference;
use reorg::conference::schema::conferences::dsl::*;
use reorg::review::model::Review;
use reorg::review::schema::reviews::dsl::*;
use reorg::reviewer::model::Reviewer;
use reorg::reviewer::schema::reviewers::dsl::*;
use reorg::submission::model::Submission;
use reorg::submission::schema::submissions::dsl::*;
use reorg::user::model::User;
use reorg::user::schema::users::dsl::*;

fn main() {
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
    
    fn generate_conference() -> Conference {
        Conference {
            id: None,
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

    fn generate_submission(conf: i32) -> Submission {
        let mut rng = rand::thread_rng();

        Submission {
            id: None,
            conference_id: conf,
            user_id: rng.gen_range(1, 11),
            status_id: rng.gen_range(1, 7),
            title: fake!(Lorem.sentence(4, 6)),
            content: fake!(Lorem.paragraph(7, 3)),
            tags: serde_json::from_str(r#"{"tags": ["Foo", "Bar", "Baz"]}"#).unwrap(),
        }
    }

    fn generate_user() -> User {
        User {
            id: None,
            first_name: fake!(Name.first_name).to_string(),
            last_name: fake!(Name.last_name).to_string(),
            email: fake!(Internet.free_email),
            password: fake!(Lorem.word).to_string(),
        }
    }

    fn generate_reviewer(conf: i32, rev: i32) -> Reviewer {
        Reviewer {
            id: None,
            conference_id: conf,
            user_id: rev,
        }
    }

    fn generate_review(rev: i32, sub: i32) -> Review {
        let mut rng = rand::thread_rng();

        Review {
            id: None,
            reviewer_id: rev,
            submission_id: sub,
            private_comments: fake!(Lorem.paragraph(7, 3)),
            shared_comments: fake!(Lorem.paragraph(7, 3)),
            score: rng.gen_range(0, 10),
        }
    }

    // Seed new data
    let mut sub_id: i32 = 0;
    println!("{}", "- Creating users".to_string());
    for _user in 0..10 {
        match User::create(&generate_user(), &connection) {
            Ok(x) => println!("-> Created user: {}", x.id.unwrap()),
            Err(y) => println!("-> Failed to create user: {}", y),
        }
    }
    println!("{}", "- Creating conferences".to_string());
    for conf_id in 1..11 {
        // Create the conferences
        match Conference::create(&generate_conference(), &connection) {
            Ok(x) => println!("-> Created conference: {}", x.id.unwrap()),
            Err(y) => println!("-> Failed to create conference: {}", y),
        }

        // Create the reviewers
        // All conferences get all users as reviewers
        println!("{}", "- Creating reviewers".to_string());
        for rev_id in 1..11 {
            match Reviewer::create(&generate_reviewer(conf_id, rev_id), &connection) {
                Ok(x) => println!("-> Created review: {}", x.id.unwrap()),
                Err(y) => println!("-> Failed to create review: {}", y),
            }
        }

        // Create the submissions and reviews
        println!("{}", "- Creating submissions".to_string());
        for _x in 1..6 {
            sub_id += 1;
            match Submission::create(&generate_submission(conf_id), &connection) {
                Ok(x) => println!("-> Created submission: {}", x.id.unwrap()),
                Err(y) => println!("-> Failed to create submission: {}", y),
            }
            println!("{}", "- Creating reviews".to_string());
            for rev_id in 1..11 {
                match Review::create(&generate_review(rev_id, sub_id), &connection) {
                    Ok(x) => println!("-> Created review: {}", x.id.unwrap()),
                    Err(y) => println!("-> Failed to create review: {}", y),
                }
            }
        }
    }
}