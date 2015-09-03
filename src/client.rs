use hyper::client;

pub fn execute_request() -> client::Response {
    //let request_str = serialize_request(request).unwrap();

    let client = client::Client::new();
    let res = client.post("http://localhost:3000")
                .body("foo=bar")
                .send()
                .unwrap();
    res
}

#[cfg(test)]
mod test {
    use super::execute_request;

    #[test]
    fn test_request() {
        //execute_request();
        println!("Request excetuted");
        assert_eq!(1, 1);
    }
}
