use async_trait::async_trait;
use tide::Request;
use serde::{Serialize, Deserialize};
use serde::de::DeserializeOwned;
use std::str::FromStr;
use std::fmt::Debug;

#[derive(Deserialize)]
#[serde(default)]
struct Page {
    size: u32,
    offset: u32,
}

impl Default for Page {
    fn default() -> Self {
        Self { size: 10, offset: 0 }
    }
}

#[async_trait]
pub trait Resourceable<S, I>: Sized + Serialize + DeserializeOwned 
    where S: Send + Sync + 'static, 
          I: FromStr + Send + Sync + 'static,
          <I as FromStr>::Err: Debug {

    async fn get(req: Request<S>) -> tide::Result {
        let id = req.param("id").unwrap().parse().unwrap();
        let resource = Self::read_by_id(req.state(), id).await?;
        let json = serde_json::to_string(&resource).unwrap();
        
        Ok(json.into())
    }

    async fn get_all(req: Request<S>) -> tide::Result {
        let page: Page = req.query()?;
        let resource = Self::read_paged(req.state(), page.size, page.offset).await?;
        let json = serde_json::to_string(&resource).unwrap();
        
        Ok(json.into())
    }

    async fn post(mut req: Request<S>) -> tide::Result {
        let request_resource: Self = req.body_json().await?;

        let new_resource = Self::create(req.state(), request_resource).await?;
        let json = serde_json::to_string(&new_resource).unwrap();
        Ok(json.into())
    }

    async fn put(mut req: Request<S>) -> tide::Result {
        let request_resource: Self = req.body_json().await?;

        let new_resource = Self::update(req.state(), request_resource).await?;
        let json = serde_json::to_string(&new_resource).unwrap();
        Ok(json.into())
    }

    async fn read_by_id(state: &S, id: I) -> anyhow::Result<Self>;

    async fn read_paged(state: &S, size: u32, offset: u32) -> anyhow::Result<Vec<Self>>;

    async fn create(state: &S, resource: Self) -> anyhow::Result<Self>;

    async fn update(state: &S, resource: Self) -> anyhow::Result<Self>;
}

#[macro_export]
macro_rules! add_resource {
    ($app:ident, $endpoint:expr, $resource:ty) => (
        $app.at($endpoint).get(<$resource>::get_all);
        $app.at(&($endpoint.to_string() + "/:id")).get(<$resource>::get);
        $app.at($endpoint).post(<$resource>::post);
    )
}

