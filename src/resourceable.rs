use async_trait::async_trait;
use anyhow::anyhow;
use tide::Request;
use serde::{Serialize, Deserialize};
use serde::de::DeserializeOwned;
use std::str::FromStr;
use std::fmt::Debug;

/// An internal struct used for extracting page requests from requests.
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

/// This trait should be implemented for each struct which has a resource you wish to access.
/// It takes two type parameters on implementation
/// - `S` - the state type used in Tide that allows resource access, often a database connection.
/// - `I` - the identity type used in your struct (e.g. i32, for GET /resource/<id>)
/// 
/// It's expected in default cases you will only implement 
/// - `read_by_id` - maps to `GET /resource/:id`
/// - `read_paged` - maps to `GET /resource?size=x&offset=y`
/// - `create` - maps to `POST /resource`
/// - `update` - maps to `PUT /resource/:id`
/// - `delete` - maps to `DELETE /resource/:id`
#[async_trait]
pub trait Resourceable<S, I>: Sized + Serialize + DeserializeOwned + Send + Sync
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
        let id = req.param("id").unwrap().parse().unwrap();
        let request_resource: Self = req.body_json().await?;

        let new_resource = Self::update(req.state(), id, request_resource).await?;
        let json = serde_json::to_string(&new_resource).unwrap();
        Ok(json.into())
    }

    async fn read_by_id(_state: &S, _id: I) -> anyhow::Result<Self> {
        Err(anyhow!("Resource not accessible by id"))
    }

    async fn read_paged(_state: &S, _size: u32, _offset: u32) -> anyhow::Result<Vec<Self>> {
        Err(anyhow!("Resource not accessible by page"))
    }

    async fn create(_state: &S, _resource: Self) -> anyhow::Result<Self> {
        Err(anyhow!("Resource not creatable!"))
    }

    async fn update(_state: &S, _id: I, _resource: Self) -> anyhow::Result<Self> {
        Err(anyhow!("Resource not updateable!"))
    }

    async fn delete(_state: &S, _id: I) -> anyhow::Result<Self> {
        Err(anyhow!("Resource not deleteable!"))
    }
}

#[macro_export]
macro_rules! add_resource {
    ($app:ident, $endpoint:expr, $resource:ty) => (
        $app.at($endpoint).get(<$resource>::get_all);
        $app.at(&($endpoint.to_string() + "/:id")).get(<$resource>::get);
        $app.at($endpoint).post(<$resource>::post);
    )
}

