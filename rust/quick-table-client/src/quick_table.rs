use serde::{Deserialize, Serialize};
use std::fmt;
use async_trait::async_trait;
//use serde_json::Result;
use std::result::Result;
use std::sync::Arc;
use std::hash::{Hash, Hasher};
use std::cmp::Ordering;

use thiserror::Error;
use std::error::Error;
use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeError;

// Error handling equivalent to QuickError
#[derive(Error,Debug)]
pub enum QuickError {
    #[error("Request error")]
    Request(#[from] ReqwestError),
    #[error("Serialization error")]
    Serialization(#[from] SerdeError),
    #[error("generic message error")]
    Message(String),
    #[error("specific error from quicktable")]
    QuickTableError(QuickTableResponse)
}

/*
// Implement Display for QuickError to improve error messages
impl fmt::Display for QuickError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Other error VX:TODO print")
    
        //match self {
         //   QuickError::Other(e) => ),
            /*
            QuickError::Message(msg) => write!(f, "Message: {}", msg),
            QuickError::None => write!(f, "None"),
            QuickError::IncorrectType { should_be, not } => write!(f, "Incorrect type: should be {:?}, not {:?}", should_be, not),
            QuickError::InvalidKey { reason } => write!(f, "Invalid key: {}", reason),
            QuickError::WrongVariantTypeRequested { value, requested_type } => write!(f, "Wrong variant type requested: value {:?}, requested type {:?}", value, requested_type),
            QuickError::RowLocked { item } => write!(f, "Row locked: {:?}", item),
            */ 
    //    }
    }
}
*/
//impl Error for QuickError {}

// Equivalent to QuickOk
#[derive(Serialize, Deserialize, Debug)]
pub struct QuickTableResponse {
    pub ok: bool,
    pub message: Option<String>,
    pub code: Option<i32>
}

impl QuickTableResponse {
    pub fn new(ok: bool, message: Option<String>, code: Option<i32>) -> Self {
        QuickTableResponse { ok, message, code }
    }
}

// Equivalent to MetaInsertResult
#[derive(Serialize, Deserialize)]
pub struct MetaInsertResult {
    pub key: i32,
}

// Equivalent to QuickCodable
#[derive(Serialize, Deserialize,Debug, Hash, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub struct QuickCodable {
    pub json: String,
}

impl QuickCodable {
    pub fn new(json: String) -> Self {
        QuickCodable { json }
    }

    pub fn to_json<T: Serialize>(codable: &T) -> Result<String, QuickError> {
        serde_json::to_string(codable).map_err(|e| QuickError::Message(format!("Failed to jsonify: {}", e)))
    }

    pub fn decode<T: for<'de> Deserialize<'de>>(&self) -> Result<T,QuickError> {
        serde_json::from_str(&self.json).map_err(|e| QuickError::Message(format!("Error decoding {} into data: {}", self.json, e)))
    }
}

// Equivalent to SortOrder
#[derive(Debug, Serialize, Deserialize)]
pub enum SortOrder {
    Ascending,
    Descending,
}

// Equivalent to QuickField
#[derive(Debug, Serialize, Deserialize)]
pub enum QuickField {
    EpochMillis,
    Score,
    Rank,
}

// Equivalent to QuickSort
pub struct QuickSort {
    pub order: SortOrder,
    pub field: QuickField,
}

// Equivalent to Comparison
#[derive(Debug, Serialize, Deserialize)]
pub enum Comparison {
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
}

// Equivalent to QuickFilter
pub struct QuickFilter {
    pub field: QuickField,
    pub comparison: Comparison,
    pub value: f64,
}

// Equivalent to QuickTableReadProtocol
#[async_trait]
pub trait QuickTableProtocol:  Send + Sync + 'static {
    async fn read(self: Arc<Self>, key: QuickKey) -> QuickReadResult;
    async fn write(self: Arc<Self>, pair: QuickPair, overwrite: Option<QuickTableOverWrite>) -> QuickTableResult;
    /*
    async fn read_bulk(self: Arc<Self>, partial_key: QuickKey, suffixes: Vec<String>) -> QuickReadBulkResult;
    async fn write(self: Arc<Self>, pair: QuickPair, overwrite: Option<QuickTableOverWrite>) -> QuickOkResult;
    async fn write_bulk(self: Arc<Self>, pairs: Vec<QuickPair>) -> QuickOkResult;
    async fn delete(self: Arc<Self>, partial_key: QuickKey) -> QuickOkResult;
    async fn scan(self: Arc<Self>, partial_key: QuickKey) -> QuickManyResult;
    async fn scan_with_options(self: Arc<Self>, partial_key: QuickKey, sort_order: Option<QuickSort>, filter: Option<QuickFilter>, max: Option<usize>) -> QuickManyResult;
    async fn count(self: Arc<Self>, partial_key: QuickKey, filter: Option<QuickFilter>) -> QuickCountResult;
    */
}


// Type aliases for results
pub type QuickReadResult = Result<QuickPair, QuickError>;
pub type QuickReadBulkResult = Result<Vec<QuickPair>, QuickError>;
pub type QuickTableResult = Result<QuickTableResponse, QuickError>;
pub type QuickManyResult = Result<Vec<QuickPair>, QuickError>;
pub type QuickCountResult = Result<QuickCount, QuickError>;

impl QuickPair {
    pub fn new(key: QuickKey, value: QuickValue, score: Option<HashableF64>, rank: Option<i32>) -> Self {
        QuickPair { key, value, score, rank }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct QuickPair {
    pub key: QuickKey,
    pub value: QuickValue,
    pub score: Option<HashableF64>,
    pub rank: Option<i32>,
}

/*
impl Ord for QuickPair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.key.cmp(&other.key)
    }
}*/

// Equivalent to QuickValue
#[derive(Serialize, Deserialize,Debug, Hash, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub struct QuickValue {
    pub codable: QuickCodable,
}

impl QuickValue {
    pub fn new_string(value: String) -> Self {
        QuickValue { codable: QuickCodable::new(value) }
    }

    pub fn new_double(value: f64) -> Self {
        QuickValue { codable: QuickCodable::new(value.to_string()) }
    }

    pub fn new_int(value: i32) -> Self {
        QuickValue { codable: QuickCodable::new(value.to_string()) }
    }

    pub fn new_codable(codable: QuickCodable) -> Self {
        QuickValue { codable }
    }

    pub fn int_value(&self) -> Option<i32> {
        self.codable.json.parse::<i32>().ok()
    }

    pub fn double_value(&self) -> Option<f64> {
        self.codable.json.parse::<f64>().ok()
    }

    pub fn string_value(&self) -> String {
        self.codable.json.clone()
    }
}

// Equivalent to QuickKey
#[derive(Debug, Hash, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct QuickKey {
    pub keys: Vec<String>,
}

impl QuickKey {
    pub fn new(keys: Vec<String>) -> Result<Self, QuickError> {
        //VX:TODO I don't know how to throw errors.
        /*
        if keys.is_empty() {
            return Err(Wtf{message:"Key length must not be zero.".to_string()});
        }*/
        Ok(QuickKey { keys })
    }

    pub fn first_key(&self) -> &String {
        &self.keys[0]
    }
}

// Implement PartialOrd and Ord for QuickKey to enable sorting
impl PartialOrd for QuickKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QuickKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let min_count = std::cmp::min(self.keys.len(), other.keys.len());
        for i in 0..min_count {
            let l_key = &self.keys[i];
            let r_key = &other.keys[i];
            if l_key < r_key {
                return std::cmp::Ordering::Less;
            } else if l_key > r_key {
                return std::cmp::Ordering::Greater;
            }
        }
        std::cmp::Ordering::Equal
    }
}

// Equivalent to QuickType
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum QuickType {
    Int,
    String,
    Codable,
    Double,
}

// Equivalent to QuickTableOverWrite
pub struct QuickTableOverWrite {
    pub new_keys: Option<QuickKey>,
}

impl QuickTableOverWrite {
    pub fn keep_key() -> Self {
        QuickTableOverWrite { new_keys: None }
    }

    pub fn with_new_key(key: QuickKey) -> Self {
        QuickTableOverWrite { new_keys: Some(key) }
    }

    pub fn no() -> Option<Self> {
        None
    }
}

// Equivalent to QuickCount
#[derive(Serialize, Deserialize)]
pub struct QuickCount {
    pub count: i32,
}


#[derive(Serialize, Deserialize,Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct HashableF64(f64);

impl Ord for HashableF64 {
    fn cmp(&self, other: &Self) -> Ordering {
        // Handle NaN cases explicitly
        if self.0.is_nan() && other.0.is_nan() {
            Ordering::Equal
        } else if self.0.is_nan() {
            Ordering::Less
        } else if other.0.is_nan() {
            Ordering::Greater
        } else {
            // Safe to unwrap here because we know the numbers are not NaN
            self.partial_cmp(other).unwrap()
        }
    }
}
impl Hash for HashableF64 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Convert f64 to bits and then hash the bits.
        self.0.to_bits().hash(state);
    }
}

impl Eq for HashableF64 {}

impl From<f64> for HashableF64 {
    fn from(val: f64) -> Self {
        HashableF64(val)
    }
}
