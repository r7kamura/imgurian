use crate::client::Client;
use crate::opt::Opt;
use crate::result::Result;

pub async fn favorite_image(opt: Opt) -> Result<()> {
    if let Opt::FavoriteImage { access_token, hash } = opt {
        let client = Client::builder().access_token(access_token).build()?;
        let basic = client.favorite_image(hash).send().await?;
        dbg!(basic);
        Ok(())
    } else {
        panic!()
    }
}
