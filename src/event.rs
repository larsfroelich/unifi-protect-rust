use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MotionEvent {
    pub id: String,
    pub model_key: String,
    pub start: i64,
    pub end: i64,
    pub score: i32,
    pub smart_detect_types: Vec<String>,
    pub camera: String,
    pub partition: Option<serde_json::Value>,
    pub user: Option<String>,
    pub metadata: MotionEventMetadata,
    pub thumbnail: Option<String>,
    pub heatmap: Option<String>,
    pub timestamp: i64,
    pub is_favorite: bool,
    pub favorite_object_ids: Option<Vec<String>>,
    pub category: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MotionEventMetadata {
    pub ram_description: Option<String>,
    pub detected_thumbnails: Option<Vec<DetectedThumbnail>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DetectedThumbnail {
    #[serde(rename = "type")]
    pub event_type: String,
    pub cropped_id: String,
    pub confidence: i32,
    pub clock_best_wall: i64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum UnifiProtectEvent {
    Motion(MotionEvent),
    #[serde(other)]
    Unknown,
}
