use std::{sync::Arc, time::SystemTime};

use dashmap::DashMap;
use ledb::{query, Collection, Document, Order, OrderKind, Primary, Storage};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sled::Db;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Document)]
struct RecordDocument {
    #[document(primary)]
    id: Option<Primary>,
    #[document(index)]
    timestamp: i64,
    #[document(index)]
    record_cloud: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Document)]
struct RangeTimeDocument {
    #[document(primary)]
    id: Option<Primary>,
    #[document(index)]
    start: i64,
    #[document(index)]
    end: i64,
    #[document(index)]
    record_cloud: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Protocol {
    pub name: Option<String>,
    pub r#type: Option<String>,
    pub path: Option<String>,
    pub port: Option<usize>,
    pub width: Option<usize>,
    pub height: Option<usize>,
}

impl Protocol {
    pub fn get_name(&self) -> &str {
        match &self.name {
            Some(v) => v,
            None => "",
        }
    }

    pub fn get_type(&self) -> &str {
        match &self.r#type {
            Some(v) => v,
            None => "",
        }
    }

    pub fn get_path(&self) -> &str {
        match &self.path {
            Some(v) => v,
            None => "",
        }
    }

    pub fn get_port(&self) -> usize {
        match &self.port {
            Some(v) => *v,
            None => 0,
        }
    }

    pub fn get_width(&self) -> usize {
        match &self.width {
            Some(v) => *v,
            None => 0,
        }
    }

    pub fn get_height(&self) -> usize {
        match &self.height {
            Some(v) => *v,
            None => 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RangeTime {
    pub start: i64,
    pub end: i64,
}

impl RangeTime {
    pub fn new() -> RangeTime {
        RangeTime { start: 0, end: 0 }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Camera {
    pub activeProtocol: Option<Protocol>,
    pub orgId: Option<String>,
    pub locationId: Option<String>,
    pub hubId: Option<String>,
    pub id: Option<String>,
    pub brand: Option<String>,
    pub name: Option<String>,
    pub model: Option<String>,
    pub status: Option<String>,
    pub record: Option<bool>,
    pub ipAddress: Option<String>,
    pub macAddress: Option<String>,
    pub serial: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub protocols: Option<Vec<Protocol>>,
    pub authenticate: Option<String>,
    pub url: Option<String>,
    #[serde(rename = "isPtzRelative")]
    pub is_ptz_relative: Option<bool>,
}

impl Camera {
    pub fn get_active_protocol(&self) -> Protocol {
        match &self.activeProtocol {
            Some(protocol) => protocol.clone(),
            None => Protocol::default(),
        }
    }

    pub fn get_organization_id(&self) -> &str {
        match &self.id {
            Some(v) => v,
            None => "",
        }
    }

    pub fn get_location_id(&self) -> &str {
        match &self.locationId {
            Some(v) => v,
            None => "",
        }
    }

    pub fn get_hub_id(&self) -> &str {
        match &self.hubId {
            Some(v) => v,
            None => "",
        }
    }

    pub fn get_id(&self) -> &str {
        match &self.id {
            Some(v) => v,
            None => "",
        }
    }

    pub fn get_brand(&self) -> &str {
        match &self.brand {
            Some(v) => v,
            None => "",
        }
    }

    pub fn get_name(&self) -> &str {
        match &self.name {
            Some(v) => v,
            None => "",
        }
    }

    pub fn get_model(&self) -> &str {
        match &self.model {
            Some(v) => v,
            None => "",
        }
    }

    pub fn get_status(&self) -> &str {
        match &self.status {
            Some(v) => v,
            None => "",
        }
    }

    pub fn is_record(&self) -> bool {
        match &self.record {
            Some(v) => *v,
            None => false,
        }
    }

    pub fn get_ip_address(&self) -> &str {
        match &self.ipAddress {
            Some(v) => v,
            None => "",
        }
    }

    pub fn get_mac_address(&self) -> &str {
        match &self.macAddress {
            Some(v) => v,
            None => "",
        }
    }

    pub fn get_serial(&self) -> &str {
        match &self.serial {
            Some(v) => v,
            None => "",
        }
    }

    pub fn get_username(&self) -> &str {
        match &self.username {
            Some(v) => v,
            None => "",
        }
    }

    pub fn get_password(&self) -> &str {
        match &self.password {
            Some(v) => v,
            None => "",
        }
    }

    pub fn get_protocols(&self) -> Vec<Protocol> {
        match &self.protocols {
            Some(v) => v.to_owned(),
            None => vec![],
        }
    }

    pub fn get_authenticate(&self) -> Option<&str> {
        match &self.authenticate {
            Some(v) => Some(v),
            None => None,
        }
    }

    pub fn get_url(&self) -> &str {
        match &self.url {
            Some(v) => v,
            None => "",
        }
    }

    pub fn is_ptz_relative(&self) -> bool {
        match &self.is_ptz_relative {
            Some(v) => *v,
            None => false,
        }
    }
}

fn main() {
    // let record_db_config = sled::Config::default()
    //     .path(format!("src/database/camera"))
    //     .mode(sled::Mode::HighThroughput);

    // let record_db = record_db_config.open().unwrap();
    // let record_db_2 = record_db.clone();

    // let mut count = 0;
    // // let mut cam_ids = vec![];

    // let res = list_query(record_db, "cameras");
    // for cam in res {
    //     let camera: Camera = serde_json::from_slice(&cam).unwrap();
    //     let status = camera.get_status();
    //     let is_record = camera.is_record();
    //     let cam_id = camera.get_id().to_owned();

    //     // if status == "active" && is_record {
    //     // cam_ids.push(cam_id);
    //     count += 1;
    //     println!("Cam id {} is record: {}", cam_id, is_record);
    //     // } else {
    //     // }
    // }
    // println!("Cam count: {:?}", count);
    let start_time_database_map: Arc<DashMap<String, Collection>> = Arc::new(DashMap::new());
    let cam_id = "8c4993f4-f044-4be3-b862-93ca872b176c".to_owned();

    let range_time_db = get_time_db(start_time_database_map.clone(), cam_id.clone());

    let filter = query!(@filter start in "1649569200000000000".."1659955599999000000");
    let found_docs: Vec<RangeTimeDocument> = range_time_db
        .find(filter, Order::Primary(OrderKind::Asc))
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let mut value_array = Vec::<i64>::new();
    for i in 0..found_docs.len() {
        let doc = &found_docs[i];
        println!("{:?}", doc);
        // value_array.push(doc.start);
    }

    // println!("{:?}", value_array);

    let now = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_nanos(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    };

    let range_time = record_specific_time_range_query(range_time_db.clone(), now as i64, 0);

    println!("range time: {:?}", range_time);

    // println!("Cam list: {:?}", cam_ids);

    // for cam_id in cam_ids {
    //     let record_db = record_db_2.clone();
    //     let cam_info_bin = &query_by_id(record_db, &cam_id)[0];

    //     let camera_info: Camera = serde_json::from_slice(&cam_info_bin).unwrap();

    //     count += 1;
    //     println!("CAM INFO: {:?}", camera_info);
    // }
    // println!("Count: {}", count);
}

pub fn query_by_id(db: Db, key: &str) -> Vec<Vec<u8>> {
    let result = match db.get(&key).unwrap() {
        Some(query) => {
            let cam: Value = serde_json::from_slice(&query.to_vec()).unwrap();
            if cam["status"].as_str().unwrap() == "active" {
                query.to_vec()
            } else {
                vec![]
            }
        }
        None => {
            vec![]
        }
    };
    vec![result]
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ErrorResponse {
    pub code: u64,
    pub message: String,
    pub status: String,
}

pub fn list_query(db: Db, object_type: &str) -> Vec<Vec<u8>> {
    let query = db.range(""..);
    let mut value_array = Vec::<Vec<u8>>::new();
    for q in query {
        if let Ok(tuple_res) = q {
            let data = tuple_res.1.to_vec();
            let data_value = serde_json::from_slice::<Value>(&data);
            if data_value.is_err() {
                continue;
            }
            if object_type == "hub" {
                value_array.push(data);
            } else {
                if data_value.unwrap()["status"].as_str().unwrap() == "active" {
                    value_array.push(data);
                }
            }
        } else {
            break;
        }
    }
    value_array
}

pub fn query_key(db: Db, key: &str) -> Value {
    let query = db.range(""..);
    let mut value_array = Vec::<Value>::new();
    for q in query {
        let tuple_res = q.unwrap();
        value_array.push(serde_json::from_slice(&tuple_res.1.to_vec()).unwrap());
    }
    // if key == "cameras" {
    //     value_array.retain(|camera| camera["status"].as_str().unwrap() == "active");
    // }
    let result = json!({ key.to_string(): json!(value_array) });
    result
}

fn get_db(
    database_map: Arc<DashMap<String, Collection>>,
    cam_id: String,
    url: String,
    date: Option<String>,
    collection_name: &str,
) -> Collection {
    let map_key = match collection_name {
        "record" => {
            format!("{}-{}", cam_id, date.unwrap())
        }
        _ => {
            format!("{}", cam_id)
        }
    };
    let database_has_key = database_map.contains_key(&map_key);
    if database_has_key == false {
        let storage = Storage::new(&url, ledb::Options::default()).unwrap();
        unsafe { storage.set_mapsize(1024 * 1024 * 1024) };

        let collection = storage.collection(collection_name).unwrap();

        match collection_name {
            "record" => {
                query!(index for collection
                    timestamp int unique,
                )
                .unwrap();
            }
            _ => {
                query!(index for collection
                    start int unique,
                    end int,
                )
                .unwrap();
            }
        }

        query!(index RecordDocument for collection).unwrap();

        query!(index RangeTimeDocument for collection).unwrap();

        database_map.insert(map_key, collection.clone());

        collection
    } else {
        let old_db = &*database_map.get(&map_key).unwrap();
        let db = old_db.clone();
        db
    }
}

fn get_time_db(database_map: Arc<DashMap<String, Collection>>, cam_id: String) -> Collection {
    let start_time_url = format!("/home/lexhub/database/record/start_time/{}", cam_id);
    let range_time_db = get_db(database_map, cam_id, start_time_url, None, "range_time");

    range_time_db
}

pub fn record_specific_time_range_query(
    collection: Collection,
    timestamp: i64,
    last_record_frame: i64,
) -> RangeTime {
    let filter = query!(@filter start <= timestamp && (end >= timestamp || end == 0));
    let found_docs: Vec<RangeTimeDocument> = collection
        .find(filter, Order::Primary(OrderKind::Asc))
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let mut result = RangeTime { start: 0, end: 0 };

    if found_docs.len() > 0 {
        let range_time = &found_docs[0];

        result.start = range_time.start;
        if range_time.end == 0 {
            result.end = last_record_frame;
        } else {
            result.end = range_time.end;
        }
    }

    result
}
