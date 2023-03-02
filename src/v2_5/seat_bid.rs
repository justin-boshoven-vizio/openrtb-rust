use super::bid::Bid;
use serde_utils;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SeatBid {
    // todo: require 1+ bid somehow
    pub bid: Vec<Bid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seat: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_utils::Ext>,
}

impl SeatBid {
    pub fn new() -> SeatBid {
        SeatBid {
            bid: vec![],
            seat: None,
            group: None,
            ext: None,
        }
    }
}

impl Default for SeatBid {
    fn default() -> Self {
        Self::new()
    }
}
