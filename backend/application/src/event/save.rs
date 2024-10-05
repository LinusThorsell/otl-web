use core::panic;

use diesel::dsl::insert_into;
use diesel::ExpressionMethods;
use diesel::RunQueryDsl;
use diesel::prelude::*;

use domain::models::User;
use domain::schema::users;
use domain::schema::users::dsl::*;

use domain::schema::scores::dsl::*;

// FIXME: fix result return type
use regex::Regex;
use rocket::serde::Deserialize;
use rocket::fs::TempFile;
use rocket::tokio::io::AsyncReadExt;

use infrastructure::establish_connection;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
struct InputCsvRecord {
    #[serde(rename = "PDGA number")]
    pdga: Option<i32>,
    #[serde(rename = "First name")]
    firstname: String,
    #[serde(rename = "Last name")]
    lastname: String,
    #[serde(rename = "Total")]
    total: i32,
    #[serde(rename = "DivCode")]
    divcode: String,
    score: Option<i32>,
}

async fn parse_file_to_csv_string(file: &TempFile<'_>) -> Result<std::string::String, Box<dyn std::error::Error>> {
    let mut contents = String::new();
    file.open().await?.read_to_string(&mut contents).await?;

    let re = Regex::new(r"(?m)^[^,\n]*$").unwrap();
    contents = re.replace_all(&contents, "").to_string().trim().to_string();

    let replace_dnf_with_999 = Regex::new(r"DNF").unwrap();
    contents = replace_dnf_with_999.replace_all(&contents, "999").to_string();

    Ok(contents)
}

fn parse_csv_string_to_records(csv_string: String) -> Result<Vec<InputCsvRecord>, Box<dyn std::error::Error>> {
    let mut records = Vec::<InputCsvRecord>::new();

    let mut reader = csv::Reader::from_reader(csv_string.as_bytes());

    for record in reader.deserialize::<InputCsvRecord>() {
        let record: InputCsvRecord = record?;
        records.push(record);
    }

    Ok(records)
}

fn assign_scores_to_records(mut records: Vec<InputCsvRecord>) -> Vec<InputCsvRecord> {
    // Sort the records by the 'total' field
    records.sort_by(|a, b| a.total.cmp(&b.total));

    let mut current_score = 100;
    let mut i = 0;

    while i < records.len() {
        let mut j = i;

        // Find the extent of the tie
        while j < records.len() && records[j].total == records[i].total {
            j += 1;
        }

        // Assign the same score to all tied records
        for k in i..j {
            records[k].score = Some(current_score);
        }

        // Decrease the score for the next position
        current_score -= (j - i) as i32;

        // Move to the next group
        i = j;
    }

    records
}

fn create_score(score_in: &i32, divcode_in: &String, event_id_in: i32, user_id_in: i32, connection: &mut PgConnection) {
    match insert_into(scores).values((score.eq(score_in), divcode.eq(divcode_in), event_id.eq(event_id_in), user_id.eq(user_id_in))).execute(connection) {
        Ok(_) => { },
        Err(_) => { }
    }
}

fn create_user(pdga_in: &Option<i32>, firstname_in: &String, lastname_in: &String) -> Result<i32, Box<dyn std::error::Error>> {
    match insert_into(users).values((pdga.eq(pdga_in), firstname.eq(firstname_in), lastname.eq(lastname_in))).get_result::<User>(&mut establish_connection()) {
        Ok(user) => { Ok(user.id) },
        Err(err) => match err {
            // TODO: Handle specific errors here.
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}

pub async fn event_parse_and_save_csv(file: &TempFile<'_>, event_id_in: i32) -> Result<(), Box<dyn std::error::Error>> {

    let csv_string = parse_file_to_csv_string(file).await?;
    let csv_records = parse_csv_string_to_records(csv_string)?;
    let scored_records = assign_scores_to_records(csv_records);

    let connection = &mut establish_connection();

    for record in scored_records {
        match users::table.select(users::all_columns).filter(pdga.eq(record.pdga)).first::<User>(connection) {
            Ok(user) => {
                create_score(&record.score.unwrap(), &record.divcode, event_id_in, user.id, connection);
            },
            Err(err) => match err {
                diesel::result::Error::NotFound => {
                    // Check if user with same firstname & lastname exists that does not have pdga
                    // number saved.
                    match users::table.select(users::all_columns).filter(pdga.is_null()).filter(firstname.eq(&record.firstname)).filter(lastname.eq(&record.lastname)).first::<User>(connection) {
                        Ok(user) => {
                            create_score(&record.score.unwrap(), &record.divcode, event_id_in, user.id, connection);
                        },
                        Err(err) => match err {
                            diesel::result::Error::NotFound => {
                                // User does not already exist, create new user.

                                let new_user_id = create_user(&record.pdga, &record.firstname, &record.lastname)?;

                                create_score(&record.score.unwrap(), &record.divcode, event_id_in, new_user_id, connection);
                            },
                            _ => {
                                panic!("Database error - {}", err);
                            }
                        }
                    }
                },
                // TODO: Handle specific errors here.
                _ => {
                    panic!("Database error - {}", err);
                }
            }
        }


    }

    Ok(())
}
