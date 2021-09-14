use chrono;
use reqwest::Client;
use reqwest::Result;
use serde::de::DeserializeOwned;
use serde::Serialize;
use sha256;

pub struct TencentCloudClient {
    ak: String,
    sk: String,
    host: String,
    service: String,
    region: String,
    version: String,
}
impl TencentCloudClient {
    pub fn new(ak: &str, sk: &str) -> Self {
        TencentCloudClient {
            ak: ak.to_string(),
            sk: sk.to_string(),
            host: "dnspod.tencentcloudapi.com".to_string(),
            service: "dnspod".to_string(),
            region: "".to_string(),
            version: "2021-03-23".to_string(),
        }
    }
    pub async fn send_request<T: DeserializeOwned, P: Serialize>(
        &self,
        method: &str,
        action: &str,
        query_without_question_mark: &str,
        body_obj: P,
    ) -> Result<T> {
        let body = serde_json::to_string(&body_obj).unwrap();

        let (ts, authorization) =
            self.make_timestamp_auth(method, query_without_question_mark, body.as_str());

        let url = format!("https://{}?{}", self.host, query_without_question_mark);

        let response = Client::new()
            .request(
                reqwest::Method::from_bytes(method.as_bytes()).unwrap(),
                reqwest::Url::parse(url.as_str()).expect("not a valid url"),
            )
            .header("Authorization", authorization)
            .header("Content-Type", "application/json; charset=utf-8")
            .header("Host", self.host.as_str())
            .header("X-TC-Action", action)
            .header("X-TC-Timestamp", ts.to_string().as_str())
            .header("X-TC-Version", self.version.as_str())
            .header("X-TC-Region", self.region.as_str())
            .body(body)
            .send()
            .await?;
        response.json().await
    }


    fn make_timestamp_auth(
        &self,
        method: &str,
        query_without_question_mark: &str,
        body: &str,
    ) -> (i64, String) {
        let content_type_json = "content-type:application/json";
        //https://cloud.tencent.com/document/product/1427/56189#Golang

        let algorithm = "TC3-HMAC-SHA256";
        let canonical_uri = "/";
        let canonical_headers =
            format!("{}; charset=utf-8\nhost:{}\n", content_type_json, self.host);

        let signed_headers = "content-type;host";
        let mut hashed_request_payload = sha256::digest("");
        if method == "POST" {
            hashed_request_payload = sha256::digest(body);
            //对于 GET 请求，RequestPayload 固定为空字符串。此示例计算结果是 35e9c5b0e3ae67532d3c9f17ead6c90222632e5b1ff7f6e89887f1398934f064。
        }
        let canonical_request = format!(
            "{}\n{}\n{}\n{}\n{}\n{}",
            method,
            canonical_uri,
            query_without_question_mark,
            canonical_headers,
            signed_headers,
            hashed_request_payload,
        );
        //println!("canonical_request        {}", canonical_request);

        // step 2: build string to sign
        //let now = chrono::Utc.ymd(2021, 7, 29).and_hms(12, 12, 12);

        let now = chrono::Utc::now();
        let timestamp = now.timestamp();
        //println!("ts:    {}", timestamp);

        let date = now.format("%Y-%m-%d").to_string();
        let credential_scope = format!("{}/{}/tc3_request", date, self.service);
        let hashed_canonical_request = sha256::digest(canonical_request);
        let string2sign = format!(
            "{}\n{}\n{}\n{}",
            algorithm, timestamp, credential_scope, hashed_canonical_request
        );

        //println!("string2sign               {}", string2sign);

        // step 3: sign string
        let secret_date = super::util::hmac_sha256(
            date.as_str().as_bytes(),
            format!("TC3{}", self.sk).as_str().as_bytes(),
        );

        let secret_service = super::util::hmac_sha256(self.service.as_bytes(), &secret_date);
        let secret_signing = super::util::hmac_sha256("tc3_request".as_bytes(), &secret_service);
        let signature =
            super::util::hmac_sha256_hex(string2sign.as_str().as_bytes(), &secret_signing);
        //println!("signature           {}", signature);

        // step 4: build authorization
        let authorization = format!(
            "{} Credential={}/{}, SignedHeaders={}, Signature={}",
            algorithm, self.ak, credential_scope, signed_headers, signature
        );
        (timestamp, authorization)
    }
}

#[cfg(test)]
mod tests {
    use super::super::model;
    use super::*;
    use tokio;


    #[tokio::test]
    async fn test_reqwest_json() {
        let ak = "your_tencent_ak";
        let sk = "your_tencent_sk";
        
        let arg = model::RecordArg {
            sub_domain: "ddddd2ds3".to_string(),
            domain: "libragen.cn".to_owned(),
            record_type: "TXT".to_owned(),
            record_line: "默认".to_owned(),
            value: "8.8.8.8".to_owned(),
        };
        //let payload = serde_json::to_string(&arg).expect("not valid json");
        //println!("arg === {}", payload);
        let s: model::ResResponseCreate = TencentCloudClient::new(ak, sk)
            .send_request("POST", "CreateRecord", "", arg)
            .await
            .expect("msg");

        println!("result === {:?}", serde_json::to_string_pretty(&s).unwrap());
    }
}
