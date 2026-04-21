use unifi_protect::UnifiProtectEvent;

#[test]
fn test_parse_motion_event() {
    let event_json = r#"{
        "id": "fa23a1d1-4d29-4c5a-a8a6-4d52df3c5739",
        "modelKey": "event",
        "type": "motion",
        "start": 1776698709469,
        "end": 1776698757185,
        "score": 100,
        "smartDetectTypes": [],
        "camera": "657bb728023b9303e4000508",
        "partition": null,
        "user": null,
        "metadata": {
            "ramDescription": "",
            "detectedThumbnails": [
                {
                    "type": "motion",
                    "croppedId": "D021F9956ADD-1776698758594",
                    "confidence": 100,
                    "clockBestWall": 1776698734205
                }
            ]
        },
        "thumbnail": "e-fa23a1d1-4d29-4c5a-a8a6-4d52df3c5739",
        "heatmap": "e-fa23a1d1-4d29-4c5a-a8a6-4d52df3c5739",
        "timestamp": 1776698733327,
        "isFavorite": false,
        "favoriteObjectIds": null,
        "category": "motion"
    }"#;

    let event: UnifiProtectEvent = serde_json::from_str(event_json).expect("Failed to parse event");

    if let UnifiProtectEvent::Motion(motion_event) = event {
        assert_eq!(motion_event.id, "fa23a1d1-4d29-4c5a-a8a6-4d52df3c5739");
        assert_eq!(motion_event.score, 100);
        assert_eq!(motion_event.camera, "657bb728023b9303e4000508");
        assert_eq!(motion_event.metadata.detected_thumbnails.unwrap().len(), 1);
    } else {
        panic!("Expected Motion event");
    }
}

#[test]
fn test_parse_unknown_event() {
    let event_json = r#"{
        "id": "unknown-id",
        "type": "someOtherType",
        "start": 123,
        "end": 456
    }"#;

    let event: UnifiProtectEvent = serde_json::from_str(event_json).expect("Failed to parse event");

    match event {
        UnifiProtectEvent::Unknown => (),
        _ => panic!("Expected Unknown event"),
    }
}
