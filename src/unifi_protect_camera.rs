use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct EventStats {
    motion: MotionStats,
    smart: SmartStats,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MotionStats {
    today: i32,
    average: i32,
    last_days: Vec<i32>,
    recent_hours: Vec<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SmartStats {
    today: i32,
    average: i32,
    last_days: Vec<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    id: i32,
    video_id: String,
    name: String,
    enabled: bool,
    is_rtsp_enabled: bool,
    rtsp_alias: Option<String>,
    width: i32,
    height: i32,
    fps: i32,
    bitrate: i32,
    min_bitrate: i32,
    max_bitrate: i32,
    min_client_adaptive_bit_rate: i32,
    min_motion_adaptive_bit_rate: i32,
    fps_values: Vec<i32>,
    idr_interval: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IspSettings {
    ae_mode: String,
    ir_led_mode: String,
    ir_led_level: i32,
    wdr: i32,
    icr_sensitivity: i32,
    brightness: i32,
    contrast: i32,
    hue: i32,
    saturation: i32,
    sharpness: i32,
    denoise: i32,
    is_color_night_vision_enabled: bool,
    is_flipped_vertical: bool,
    is_flipped_horizontal: bool,
    is_auto_rotate_enabled: bool,
    is_ldc_enabled: bool,
    is3dnr_enabled: bool,
    is_external_ir_enabled: bool,
    is_aggressive_anti_flicker_enabled: bool,
    is_pause_motion_enabled: bool,
    d_zoom_center_x: i32,
    d_zoom_center_y: i32,
    d_zoom_scale: i32,
    d_zoom_stream_id: i32,
    focus_mode: String,
    focus_position: i32,
    touch_focus_x: i32,
    touch_focus_y: i32,
    zoom_position: i32,
    mount_position: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TalkbackSettings {
    type_fmt: String,
    type_in: String,
    bind_addr: String,
    bind_port: i32,
    filter_addr: String,
    filter_port: i32,
    channels: i32,
    sampling_rate: i32,
    bits_per_sample: i32,
    quality: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OsdSettings {
    is_name_enabled: bool,
    is_date_enabled: bool,
    is_logo_enabled: bool,
    is_debug_enabled: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LedSettings {
    is_enabled: bool,
    blink_rate: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeakerSettings {
    is_enabled: bool,
    are_system_sounds_enabled: bool,
    volume: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordingSettings {
    pre_padding_secs: i32,
    post_padding_secs: i32,
    min_motion_event_trigger: i32,
    end_motion_event_delay: i32,
    suppress_illumination_surge: bool,
    mode: String,
    geofencing: String,
    motion_algorithm: String,
    enable_motion_detection: bool,
    enable_pir_timelapse: bool,
    use_new_motion_algorithm: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PIRSettings {
    pir_sensitivity: i32,
    pir_motion_clip_length: i32,
    timelapse_frame_interval: i32,
    timelapse_transfer_interval: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WifiConnectionState {
    channel: Option<i32>,
    frequency: Option<i32>,
    phy_rate: Option<f32>,
    tx_rate: Option<f32>,
    signal_quality: Option<i32>,
    ssid: Option<String>,
    bssid: Option<String>,
    ap_name: Option<String>,
    experience: Option<String>,
    signal_strength: Option<i32>,
    connectivity: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamSharing {
    enabled: bool,
    token: Option<String>,
    share_link: Option<String>,
    expires: Option<i64>,
    shared_by_user_id: Option<String>,
    shared_by_user: Option<String>,
    max_streams: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HomekitSettings {
    talkback_settings_active: bool,
    stream_in_progress: bool,
    microphone_muted: bool,
    speaker_muted: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    rx_bytes: i64,
    tx_bytes: i64,
    wifi: WifiStats,
    battery: BatteryStats,
    video: VideoStats,
    storage: StorageStats,
    wifi_quality: i32,
    wifi_strength: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WifiStats {
    channel: i32,
    frequency: i32,
    link_speed_mbps: Option<f32>,
    signal_quality: i32,
    signal_strength: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatteryStats {
    percentage: Option<i32>,
    is_charging: bool,
    sleep_state: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoStats {
    recording_start: i64,
    recording_end: i64,
    #[serde(alias = "recordingStartLQ")]
    recording_start_lq: i64,
    #[serde(alias = "recordingEndLQ")]
    recording_end_lq: i64,
    timelapse_start: i64,
    timelapse_end: i64,
    #[serde(alias = "timelapseStartLQ")]
    timelapse_start_lq: i64,
    #[serde(alias = "timelapseEndLQ")]
    timelapse_end_lq: i64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageStats {
    used: i64,
    rate: f64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FeatureFlags {
    can_adjust_ir_led_level: bool,
    can_magic_zoom: bool,
    can_optical_zoom: bool,
    can_touch_focus: bool,
    has_accelerometer: bool,
    has_aec: bool,
    #[serde(alias = "hasAutoICROnly")]
    has_auto_icronly: bool,
    has_battery: bool,
    has_bluetooth: bool,
    has_chime: bool,
    has_external_ir: bool,
    has_icr_sensitivity: bool,
    has_infrared: bool,
    has_ldc: bool,
    has_led_ir: bool,
    has_led_status: bool,
    has_line_in: bool,
    has_mic: bool,
    has_privacy_mask: bool,
    has_rtc: bool,
    has_sd_card: bool,
    has_speaker: bool,
    has_wifi: bool,
    has_hdr: bool,
    video_modes: Vec<String>,
    video_mode_max_fps: Vec<HashMap<String, i32>>,
    has_motion_zones: bool,
    has_lcd_screen: bool,
    mount_positions: Vec<String>,
    smart_detect_types: Vec<String>,
    smart_detect_audio_types: Vec<String>,
    lens_type: Option<String>,
    lens_model: Option<String>,
    motion_algorithms: Vec<String>,
    has_square_event_thumbnail: bool,
    has_package_camera: bool,
    audio: Vec<String>,
    audio_codecs: Vec<String>,
    is_doorbell: bool,
    privacy_mask_capability: PrivacyMaskCapability,
    focus: FocusSettings,
    pan: PanSettings,
    tilt: TiltSettings,
    zoom: ZoomSettings,
    hotplug: HotplugSettings,
    has_smart_detect: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PrivacyMaskCapability {
    max_masks: i32,
    rectangle_only: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FocusSettings {
    steps: FocusSteps,
    degrees: FocusDegrees,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FocusSteps {
    max: Option<i32>,
    min: Option<i32>,
    step: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FocusDegrees {
    max: Option<i32>,
    min: Option<i32>,
    step: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PanSettings {
    steps: PanSteps,
    degrees: PanDegrees,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PanSteps {
    max: Option<i32>,
    min: Option<i32>,
    step: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PanDegrees {
    max: Option<i32>,
    min: Option<i32>,
    step: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TiltSettings {
    steps: TiltSteps,
    degrees: TiltDegrees,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TiltSteps {
    max: Option<i32>,
    min: Option<i32>,
    step: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TiltDegrees {
    max: Option<i32>,
    min: Option<i32>,
    step: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ZoomSettings {
    steps: ZoomSteps,
    degrees: ZoomDegrees,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ZoomSteps {
    max: Option<i32>,
    min: Option<i32>,
    step: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ZoomDegrees {
    max: Option<i32>,
    min: Option<i32>,
    step: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HotplugSettings {
    audio: Option<bool>,
    video: Option<bool>,
    extender: Option<ExtenderSettings>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtenderSettings {
    is_attached: bool,
    has_flash: Option<bool>,
    has_ir: Option<bool>,
    has_radar: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SmartDetectSettings {
    object_types: Vec<String>,
    auto_tracking_object_types: Vec<String>,
    audio_types: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RecordingSchedule {
    // fields unknown
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MotionZone {
    // fields unknown
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PrivacyZone {
    // fields unknown
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SmartDetectZone {
    // fields unknown
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SmartDetectLine {
    // fields unknown
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CameraFocus {
    steps: CameraSteps,
    degrees: CameraDegrees,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CameraPan {
    steps: CameraSteps,
    degrees: CameraDegrees,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CameraTilt {
    steps: CameraSteps,
    degrees: CameraDegrees,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CameraZoom {
    steps: CameraSteps,
    degrees: CameraDegrees,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CameraSteps {
    max: Option<i32>,
    min: Option<i32>,
    step: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CameraDegrees {
    max: Option<i32>,
    min: Option<i32>,
    step: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Hotplug {
    audio: Option<bool>,
    video: Option<bool>,
    extender: Extender,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Extender {
    is_attached: bool,
    has_flash: Option<bool>,
    has_ir: Option<bool>,
    has_radar: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WiredConnectionState {
    phy_rate: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UnifiProtectCamera {
    pub is_deleting: bool,
    pub mac: String,
    pub host: String,
    pub name: String,
    pub connection_host: String,
    pub connected_since: Option<i64>,
    pub state: String,
    pub hardware_revision: String,
    pub firmware_version: String,
    pub latest_firmware_version: String,
    pub firmware_build: String,
    pub is_updating: bool,
    #[serde(alias = "isDownloadingFW")]
    pub is_downloading_fw: bool,
    pub fw_update_state: String,
    pub is_adopting: bool,
    pub is_adopted: bool,
    pub is_adopted_by_other: bool,
    pub is_provisioned: bool,
    pub is_rebooting: bool,
    pub is_ssh_enabled: bool,
    pub can_adopt: bool,
    pub is_attempting_to_connect: bool,
    pub guid: Option<String>,
    pub last_motion: i64,
    pub mic_volume: i32,
    pub is_mic_enabled: bool,
    pub is_recording: bool,
    pub is_wireless_uplink_enabled: bool,
    pub is_motion_detected: bool,
    pub is_smart_detected: bool,
    pub phy_rate: f64,
    pub hdr_mode: bool,
    pub video_mode: String,
    pub is_probing_for_wifi: bool,
    pub ap_mac: Option<String>,
    pub ap_rssi: Option<i32>,
    pub ap_mgmt_ip: Option<String>,
    pub element_info: Option<String>,
    pub chime_duration: i32,
    pub is_dark: bool,
    pub last_privacy_zone_position_id: Option<i64>,
    pub last_ring: Option<i64>,
    pub is_live_heatmap_enabled: bool,
    pub anonymous_device_id: String,
    pub event_stats: EventStats,
    pub video_reconfiguration_in_progress: bool,
    pub voltage: Option<i32>,
    pub use_global: bool,
    pub is_poor_network: bool,
    pub stop_stream_level: Option<i32>,
    pub is_waterproof_case_attached: bool,
    pub last_disconnect: i64,
    pub user_configured_ap: bool,
    pub wired_connection_state: WiredConnectionState,
    pub channels: Vec<Channel>,
    pub isp_settings: IspSettings,
    pub talkback_settings: TalkbackSettings,
    pub osd_settings: OsdSettings,
    pub led_settings: LedSettings,
    pub speaker_settings: SpeakerSettings,
    pub recording_settings: RecordingSettings,
    pub smart_detect_settings: SmartDetectSettings,
    pub recording_schedules: Vec<RecordingSchedule>,
    pub motion_zones: Vec<MotionZone>,
    pub privacy_zones: Vec<PrivacyZone>,
    pub smart_detect_zones: Vec<SmartDetectZone>,
    pub smart_detect_lines: Vec<SmartDetectLine>,
    pub stats: Stats,
    pub feature_flags: FeatureFlags,
    pub pir_settings: PIRSettings,
    pub lcd_message: HashMap<String, String>,
    pub wifi_connection_state: WifiConnectionState,
    pub lenses: Vec<String>,
    pub stream_sharing: StreamSharing,
    pub homekit_settings: HomekitSettings,
    pub id: String,
    pub nvr_mac: String,
    pub is_connected: bool,
    pub platform: String,
    pub has_speaker: bool,
    pub has_wifi: bool,
    pub audio_bitrate: i32,
    pub can_manage: bool,
    pub is_managed: bool,
    pub market_name: String,
    #[serde(alias = "is4K")]
    pub is4k: bool,
    #[serde(alias = "is2K")]
    pub is2k: bool,
    pub model_key: String,
}

#[cfg(test)]
mod tests {
    use crate::unifi_protect_camera::UnifiProtectCamera;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn main() {
        // Open the JSON file
        let mut file = File::open("./src/sample_data/cameras.json").expect("Failed to open file");
        // Read the file contents into a String
        let mut cameras_json = String::new();
        file.read_to_string(&mut cameras_json)
            .expect("Failed to read file");

        // Parse the JSON data into the UnifiProtectCamera pub struct using Serde
        let cameras: Vec<UnifiProtectCamera> = serde_json::from_str(&cameras_json).unwrap();

        // Print the UnifiProtectCamera pub struct
        println!("{:?}", cameras);
    }
}
