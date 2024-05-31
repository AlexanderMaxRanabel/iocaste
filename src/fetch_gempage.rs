use trotter::{Actor, UserAgent};

pub async fn mk_req(mut url: String) -> anyhow::Result<String> {
    if !url.ends_with("g") && !url.ends_with(".gmi") {
        url = format!("{}/", url);
    }

    let requester = Actor::default().user_agent(UserAgent::Archiver);

    let response = requester.get(url).await?.gemtext()?;

    Ok(response)
}
