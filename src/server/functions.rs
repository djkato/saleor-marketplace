use leptos::*;

use super::types::SaleorApp;

#[server]
pub async fn get_all_apps() -> Result<Vec<SaleorApp>, ServerFnError>{
    let conn = crate::server::db::connect().await?;
    let mut res = conn.query("SELECT * FROM saleor_app").await?;

    let apps :Vec<SaleorApp> = res.take(0)?;
    // dbg!(&apps);
    Ok(apps)
}



