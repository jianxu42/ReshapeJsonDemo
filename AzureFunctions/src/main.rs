use serde::{Deserialize, Serialize};
use std::{convert::Infallible, env, net::Ipv4Addr};
use warp::Filter;

pub type DataVec = Vec<Data>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    #[serde(rename = "RowID")]
    pub row_id: Option<i64>,
    #[serde(rename = "ID")]
    pub id: i64,
    #[serde(rename = "Title")]
    pub title: String,
}

impl Data {
    pub fn new(row_id: Option<i64>, id: i64, title: String) -> Self {
        Self { row_id, id, title }
    }
}

fn json_body() -> impl Filter<Extract = (DataVec,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16000).and(warp::body::json())
}

pub async fn update_body(
    old_data_vec: DataVec,
    mut new_data_vec: DataVec,
) -> Result<impl warp::Reply, Infallible> {
    let mut i: i64 = 1;
    for old_data in old_data_vec.iter() {
        new_data_vec.append(&mut vec![Data::new(
            Some(i),
            old_data.id,
            old_data.title.clone(),
        )]);
        i += 1;
    }

    Ok(warp::reply::json(&new_data_vec))
}

#[tokio::main]
async fn main() {
    let new_data_list = DataVec::default();
    let new_data_list_filter = warp::any().map(move || new_data_list.clone());
    let route_addrowid = warp::post()
        .and(warp::path("api"))
        .and(warp::path("addrowid"))
        .and(json_body())
        .and(new_data_list_filter)
        .and_then(update_body);
    let route_echo = warp::get()
        .and(warp::path("api"))
        .and(warp::path("echo"))
        .and(warp::path::end())
        .map(|| "Hello, world!");
    let route = route_addrowid.or(route_echo);

    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    warp::serve(route).run((Ipv4Addr::LOCALHOST, port)).await
}
