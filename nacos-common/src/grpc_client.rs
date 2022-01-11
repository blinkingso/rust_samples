pub mod request {
    use std::collections::HashMap;

    pub trait Request {
        fn put_header(&mut self, key: String, value: String);

        fn put_all_headers(&mut self, headers: HashMap<String, String>);

        fn get_header(&self, key: String) -> Option<String>;

        fn get_header_with_default(&self, key: String, default_value: String);

        fn get_request_id(&self) -> String;

        fn set_request_id(&mut self, request_id: String);

        fn get_headers(&self) -> HashMap<String, String>;

        fn clear_headers(&mut self);
    }

    struct Headers {
        pub(crate) headers: HashMap<String, String>,
        pub(crate) request_id: String,
    }

    impl Request for Headers {
        fn put_header(&mut self, key: String, value: String) {
            self.headers.insert(key, value);
        }

        fn put_all_headers(&mut self, headers: HashMap<String, String>) {
            self.headers.extend(headers);
        }

        fn get_header(&self, key: String) -> Option<String> {
            self.headers.get(&key).map(|v| v.clone())
        }

        fn get_header_with_default(&self, key: String, default_value: String) -> String {
            self.headers.get(&key).map_or(default_value, |v| v.clone())
        }

        fn get_request_id(&self) -> String {
            self.request_id.clone()
        }

        fn set_request_id(&mut self, request_id: String) {
            self.request_id = request_id;
        }

        fn get_headers(&self) -> HashMap<String, String> {
            self.headers.clone()
        }

        fn clear_headers(&mut self) {
            self.headers.clear()
        }
    }

    pub trait InternalRequest: Request {
        fn get_module(&self) -> String {
            String::from("internal")
        }
    }

    pub struct ClientAbilities {}

    pub struct ConnectionSetupRequest {
        headers: Headers,
        client_version: String,
        abilities: ClientAbilities,
        tenant: String,
        labels: HashMap<String, String>,
    }

    impl InternalRequest for ConnectionSetupRequest {}
    impl Request for ConnectionSetupRequest {
        fn put_header(&mut self, key: String, value: String) {
            self.headers.put_header(key, value)
        }

        fn put_all_headers(&mut self, headers: HashMap<String, String>) {
            self.headers.put_all_headers(headers);
        }

        fn get_header(&self, key: String) -> Option<String> {
            self.headers.get_header(key)
        }

        fn get_header_with_default(&self, key: String, default_value: String) -> String {
            self.headers.get_header_with_default(key, default_value)
        }

        fn get_request_id(&self) -> String {
            self.headers.get_request_id()
        }

        fn set_request_id(&mut self, request_id: String) {
            self.headers.set_request_id(request_id);
        }

        fn get_headers(&self) -> HashMap<String, String> {
            self.headers.get_headers()
        }

        fn clear_headers(&mut self) {
            self.headers.clear_headers();
        }
    }
}
