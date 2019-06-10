extern crate chrono;
extern crate reorg;
extern crate diesel;
#[macro_use] 
extern crate fake;

use chrono::NaiveDate;
use reorg::*;
use reorg::models::NewConference;

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

    for _x in 0..100 {
        create_conference(&connection, &generate_conference());
    }
}