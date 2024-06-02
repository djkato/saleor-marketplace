use std::path::PathBuf;

use saleor_app_sdk::manifest::AppManifest;
use serde::Deserialize;
use surrealdb::engine::local::Db;
use surrealdb::engine::local::RocksDb;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

use super::types::SaleorApp;

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[tokio::main]
pub async fn connect() -> surrealdb::Result<Surreal<Db>> {
    // Create database connection

    let db = Surreal::new::<RocksDb>("/db").await?;

    // Select a specific namespace / database
    db.use_ns("marketplace").use_db("apps").await?;

    let demo_data: Vec<Record> = db.create("saleor_app").content(SaleorApp{
        name: "Sitemap generator".to_owned(),
        logo: PathBuf::new(),
        images: vec![PathBuf::new()],
        authors: vec![],
        versions: vec![semver::Version::parse( "2.1.1").unwrap()],
        description: "Sitemap stuff".to_owned(),
        is_verified: true,
        last_updated: chrono::Local::now().into(),
        built_for_url: None,
        supported_apls: vec![],
        supported_deployments: vec![],
        minimal_docker_compose: "".to_owned(),
        manifest: AppManifest{
            name: "Sitemap generator".to_owned(),
            id: "hihihaha".to_owned(),
            about: None,
            brand: None,
            author: None,
            version: "2.1.1".to_owned(),
            app_url: "localhost:3000".to_owned(),
            token_target_url: "localhost:3000".to_owned(),
            ..Default::default()
        }
    }).await?;

    dbg!(demo_data);
    // // Create a new person with a random id
    // let created: Vec<Record> = db
    //     .create("person")
    //     .content(Person {
    //         title: "Founder & CEO",
    //         name: Name {
    //             first: "Tobie",
    //             last: "Morgan Hitchcock",
    //         },
    //         marketing: true,
    //     })
    //     .await?;
    // dbg!(created);
    //
    // // Update a person record with a specific id
    // let updated: Option<Record> = db
    //     .update(("person", "jaime"))
    //     .merge(Responsibility { marketing: true })
    //     .await?;
    // dbg!(updated);
    //
    // // Select all people records
    // let people: Vec<Record> = db.select("person").await?;
    // dbg!(people);
    //
    // // Perform a custom advanced query
    // let groups = db
    //     .query("SELECT marketing, count() FROM type::table($table) GROUP BY marketing")
    //     .bind(("table", "person"))
    //     .await?;
    // dbg!(groups);
    //
    Ok(db)
}
