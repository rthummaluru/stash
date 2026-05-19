use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct FileEvent {
    pub id: String,
    pub file_name: String,
    pub full_path: String,
    pub source_folder: String,
    pub detected_at: String,
}
