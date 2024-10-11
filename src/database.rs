use serde::{Deserialize, Serialize};
use serde_json;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::opt::Resource;
use surrealdb::RecordId;
use surrealdb::Surreal;
use surrealdb::Value;

#[derive(Debug, Serialize)]
pub struct Name<'a> {
    pub first: &'a str,
    pub last: &'a str,
}

#[derive(Debug, Serialize)]
pub struct Person<'a> {
    pub title: &'a str,
    pub name: Name<'a>,
    pub marketing: bool,
}

#[derive(Debug, Serialize)]
pub struct Responsibility {
    pub marketing: bool,
}

#[derive(Debug, Deserialize)]
pub struct Record {
    id: RecordId,
}

#[tokio::main]
pub async fn dodb() -> surrealdb::Result<String> {
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;

    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    db.use_ns("test").use_db("test").await?;

    // Create a new person with a random id
    let created: Option<Record> = db
        .create("person")
        .content(Person {
            title: "Founder & CEO",
            name: Name {
                first: "Dennis",
                last: "The Menace",
            },
            marketing: true,
        })
        .await?;
    //dbg!(created);

    // Update a person record with a specific id
    // We don't care about the response in this case
    // so we are just going to use `Resource::from`
    // to let the compiler return `surrealdb::Value`
    db.update(Resource::from(("person", "jaime")))
        .merge(Responsibility { marketing: true })
        .await?;

    // Select all people records
    let people: Vec<Record> = db.select("person").await?;
    //dbg!(people);

    // Perform a custom advanced query
    let mut groups = db
        .query("SELECT marketing, count() FROM type::table($table) GROUP BY marketing")
        .bind(("table", "person"))
        .await?;
    // Use .take() to transform the first query result into
    // anything that can be deserialized, in this case
    // a Value
    // dbg!(groups.take::<Value>(0).unwrap());

    // Use .take() to deserialize the first query result as a Value
    let res_value = groups.take::<Value>(0).unwrap();

    // Convert Value to JSON using serde_json
    let res_json = serde_json::to_string(&res_value).unwrap();

    Ok(res_json)
}
