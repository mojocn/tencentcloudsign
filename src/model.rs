use serde;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RecordArg {
    pub domain: String,
    pub sub_domain: String,
    pub record_type: String,
    pub record_line: String,
    pub value: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResResponseCreate {
    pub response: ResResponseCreateContent,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResResponseCreateContent {
    pub error: Option<ResResponseError>,
    pub request_id: String,
    pub record_id: Option<i64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResResponseError {
    pub code: Option<String>,
    pub message: Option<String>,
}

#[cfg(test)]
mod tests {
   use super::*;

    #[test]
    fn test_json_model() {
        let raw = r#"{"Response":{"Error":{"Code":"InvalidParameter.DomainRecordExist","Message":"记录已经存在，无需再次添加。"},"RequestId":"311b7561-cc89-493e-8fce-151e39a2af97"}}"#;
        let _mode: ResResponseCreate = serde_json::from_str(raw).expect("msg");
        let raw = r#"{"Response":{"RecordId":891172194,"RequestId":"6d692211-98f2-4b33-ae7a-ac270386a450"}}"#;
        let _mode: ResResponseCreate = serde_json::from_str(raw).expect("msg");
    }
}
