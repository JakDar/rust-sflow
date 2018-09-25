use header_record::layer7::http::{HttpRequest, HttpResponse};

#[derive(Debug, Clone, PartialEq)]
pub enum Layer7Packet {
    HttpReq(HttpRequest),
    HttpResp(HttpResponse),
    Unknown,
}

impl Default for Layer7Packet {
    fn default() -> Self {
        Layer7Packet::Unknown
    }
}

impl Layer7Packet {
    //TODO - clean
    pub fn decode(l7_body: &str) -> Layer7Packet {
        let lines: Vec<&str> = l7_body.split("\n").collect();

        if lines.len() < 1 {
            Layer7Packet::Unknown
        } else {
            let line1: Vec<&str> = lines.get(0).unwrap().split(" ").collect();

            if line1.len() < 3 {
                Layer7Packet::Unknown
            } else {
                let t1 = line1.first().unwrap();
                let t2: &str = line1.get(1).unwrap();

                if t1.starts_with("HTTP") {
                    match t2.parse::<i32>() {
                        Ok(status_code) =>
                            Layer7Packet::HttpResp(HttpResponse::new(status_code)),
                        _ => Layer7Packet::Unknown
                    }
                } else {
                    let line2: &str = lines.get(1).unwrap().trim();
                    let host = if line2.starts_with("Host:") {
                        println!("Here");
                        let vec: Vec<&str> = line2.split(": ").collect();
                        vec.last().map(|x| x.clone())
                    } else {
                        Option::None
                    };


                    Layer7Packet::HttpReq(HttpRequest::new(t1.to_string(), t2.to_string(), host.map(|x| x.to_string())))
                }
            }
        }
    }
}
