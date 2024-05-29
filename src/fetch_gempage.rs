use trotter::{Actor, UserAgent};

pub async fn mk_req(url: String) -> anyhow::Result<String> {
    let requester = Actor::default().user_agent(UserAgent::Archiver);

    let response = requester.get(url).await?.gemtext()?;

    Ok(response)
}
