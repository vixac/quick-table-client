use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;
use std::sync::Arc;
use async_trait::async_trait;

use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeError;
use thiserror::Error;

use crate::quick_table::{
    QuickFilter,
    QuickTableProtocol,
    QuickReadResult,
    QuickKey,
    QuickValue,
    QuickPair,
    QuickTableResponse,
    QuickTableOverWrite,
    QuickReadBulkResult,
    QuickTableResult,
    QuickManyResult,
    QuickCountResult,
    QuickError,
    HashableF64,
};

pub struct QuickHandle {
    pub endpoint: String,
    pub client: reqwest::Client
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct UpdateRequest {
    row:  QuickRowCodable,
    overwrite: bool,
    newKeys: Option<Vec<String>>,
}

impl UpdateRequest {
    fn new(pair: QuickPair, overwrite: Option<QuickTableOverWrite>) -> UpdateRequest {
        let row = QuickRowCodable {
            keys: pair.key.keys,
            rank: pair.rank,
            codable: pair.value.codable.json,
            score: pair.score};

        let overwrite_bool = match overwrite {
            Some(_) => true,
            None => false,
        };
        //nested match. if theres overwrite and also theres new keys, then new keys, otherwise none.
        let new_keys: Option<Vec<String>> = match overwrite {
            Some(val) => match val.new_keys {
                Some(quick_key) => Some(quick_key.keys),
                None => None,
            },
            None => None
        };
        UpdateRequest { row: row, overwrite: overwrite_bool, newKeys: new_keys}
    }
}



#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct QuickRowCodable {
    keys: Vec<String>,
    rank: Option<i32>,
    codable: String,
    score: Option<HashableF64>,
}

#[async_trait]
impl QuickTableProtocol for QuickHandle {
    async fn read(self: Arc<Self>, key: QuickKey) -> QuickReadResult {

        let url =  format!("{}/scan", self.endpoint);
        let response = self.client.post(url)
            .json(&key)
            .send()
            .await?;

        let request_result = response.json::<QuickPair>().await;
        let response: Result<QuickPair,QuickError> = match request_result {
            Ok(body) => Ok(body),
            Err(e) => Err(QuickError::Message(format!("VX: Error in response: {}", e)))
        };
        response
        
    }

    async fn write(self: Arc<Self>, pair: QuickPair, overwrite: Option<QuickTableOverWrite>) -> QuickTableResult {
        let url =  format!("{}/update", self.endpoint);
        let req = UpdateRequest::new(pair, overwrite);
        let request_result = self.client.post(url)
            .json(&req)
            .send()
            .await?;
        let ok_response = request_result.json::<QuickTableResponse>().await;
        println!("write result from quicktable is {:?}", ok_response);
        let response: Result<QuickTableResponse,QuickError> = match ok_response {
            Ok(body) => Ok(body),
            Err(e) => Err(QuickError::Message(format!("VX: Error in response: {}", e)))
        };
        response
    }
}

//this is a mock quick pair
        /*
        let vec =  vec!["ok".to_string()];
        let key =  QuickKey{ keys: vec };
        let value = QuickValue::new_string("hi".to_string());
        let pair = QuickPair{key: key, value: value, score: None, rank: None};
        Ok(pair)
        */