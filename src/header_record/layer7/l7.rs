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
            let line1: Vec<&str> = lines.get(0).unwrap_or_else(|| panic!("rust-sflow l7 - lines are empty")).split(" ").collect();

            if line1.len() < 3 {
                Layer7Packet::Unknown
            } else {
                let t1 = line1.first().unwrap_or_else(|| panic!("rust-sflow l7 - line1 has <1 elems"));
                let t2: &str = line1.get(1).unwrap_or_else(|| panic!("rust-sflow l7 - line has <2 elems"));

                if t1.starts_with("HTTP") {
                    match t2.parse::<i32>() {
                        Ok(status_code) =>
                            Layer7Packet::HttpResp(HttpResponse::new(status_code)),
                        _ => Layer7Packet::Unknown
                    }
                } else if vec!["GET", "POST", "OPTIONS", "HEAD", "CONNECT", "PATCH", "PUT", "DELETE", "TRACE", "PATCH"].contains(t1)
                    {
                        //todo:if doesnt start with http then error?
                        let host: Option<&str> = lines.get(1).and_then(|line2| {
                            if line2.starts_with("Host:") {
                                let vec: Vec<&str> = line2.split(": ").collect();
                                vec.last().map(|x| x.clone())
                            } else {
                                Option::None
                            }
                        });


                        Layer7Packet::HttpReq(HttpRequest::new(t1.to_string(), t2.to_string(), host.map(|x| x.to_string())))
                    } else {
                    Layer7Packet::Unknown
                }
            }
        }
    }
}
