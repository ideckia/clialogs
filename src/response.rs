use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    #[serde(rename = "type")]
    response_type: ResponseType,
    body: Vec<ResponseBody>,
}

impl Response {
    pub fn ok(body: Vec<ResponseBody>) {
        Response {
            response_type: ResponseType::Ok,
            body: body,
        }
        .print();
    }

    pub fn cancel() {
        Response {
            response_type: ResponseType::Cancel,
            body: Vec::new(),
        }
        .print();
    }

    fn print(&self) {
        match serde_json::to_string(&self) {
            Ok(s) => {
                println!("{}", s)
            }
            Err(e) => {
                println!("Error serializing ResponseBodys {}", e)
            }
        }
        std::process::exit(0);
    }
}

#[derive(Serialize)]
pub struct ResponseBody {
    pub id: String,
    pub value: String,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
enum ResponseType {
    Ok,
    Cancel,
}
