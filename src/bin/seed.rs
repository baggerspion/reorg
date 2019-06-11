extern crate chrono;
extern crate diesel;
#[macro_use] 
extern crate fake;
extern crate rand;
extern crate reorg;

use chrono::NaiveDate;
use rand::Rng;
use reorg::*;
use reorg::models::*;

fn main() {
    // Clean out old test data
    let connection = establish_connection();
    
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

    fn generate_submission() -> NewSubmission {
        let mut rng = rand::thread_rng();

        NewSubmission {
            conference_id: rng.gen_range(1, 11),
            user_id: rng.gen_range(1, 21),
            title: fake!(Lorem.sentence(4, 6)),
            content: fake!(Lorem.paragraph(7,3)),
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

    pub fn generate_reviewer() -> NewReviewer {
        let mut rng = rand::thread_rng();

        NewReviewer {
            conference_id: rng.gen_range(1, 11),
            user_id: rng.gen_range(1, 21),
        }
    }

    for _x in 0..20 {
        create_user(&connection, &generate_user());
    }
    for _y in 0..10 {
        create_conference(&connection, &generate_conference());
    }
    for _z in 1..100 {
        create_submission(&connection, &generate_submission());
    }
    for _a in 1..10 {
        create_reviewer(&connection, &generate_reviewer());
    }
}