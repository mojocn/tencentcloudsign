mod model;
mod txcloud;
mod util;

#[tokio::main]
async fn main() {
    let ak = "your_tencent_ak";
    let sk = "your_tencent_sk";

    //https://cloud.tencent.com/document/product/1427/56179
    let arg = model::RecordArg {
        sub_domain: "5x68".to_string(),
        domain: "libragen.cn".to_owned(),
        record_type: "TXT".to_owned(),
        record_line: "默认".to_owned(),
        value: "8.8.8.8".to_owned(),
    };

    let result_instance:model::ResResponseCreate = txcloud::TencentCloudClient::new(ak, sk)
        .send_request("POST", "CreateRecord", "", arg)
        .await
        .expect("msg");


    let payload = serde_json::to_string(&result_instance).expect("not valid json");

    println!("result === {:?}", payload);
}
