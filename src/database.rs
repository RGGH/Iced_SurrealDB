use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::{Error, RecordId, Surreal};

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
    pub company : String,
}

#[derive(Debug, Serialize)]
pub struct Responsibility {
    pub marketing: bool,
}

#[derive(Debug, Deserialize)]
pub struct Record {
    id: RecordId,
}

#[derive(Debug, Deserialize)]
pub struct Marketing {
    pub count : i32,
    pub marketing : bool,
    pub company : String,
}

#[tokio::main]
pub async fn dodb() -> Result<String, Error> {
    // Connect to the database
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;

    // Sign in as root user
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    // Select the namespace and database
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
            company : "jml".to_string(),
        })
        .await?;
    dbg!(created);

    // Perform a custom advanced query
    let mut entries = db
        .query("SELECT marketing, company, count() FROM type::table($table) GROUP BY marketing")
        .bind(("table", "person"))
        .await?;

    dbg!(&entries);
    let entries: Vec<Marketing> = entries.take(0)?;
    for entry in &entries {
        println!("Count {:?} Marketing? {:?} Company {:?}", entry.count, entry.marketing, entry.company);
    }

    let result_string = entries[0].company.to_string();
    Ok(result_string) // Return the result string
}

