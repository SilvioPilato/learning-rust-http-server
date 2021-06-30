use std::collections::HashMap;

#[derive(Debug)]
pub struct Headers<'buf> {
    data: HashMap<&'buf str, &'buf str>
}

impl<'buf> Headers<'buf> {
    pub fn get(&self, key: &str) -> Option<&&str>{
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for Headers<'buf> {
    fn from(headers: &'buf str) -> Self { 
        let mut data = HashMap::new();
        for sub_str in headers.split("\r\n") {
            let mut key = sub_str;
            let mut val = "";
            if let Some(i) = sub_str.find(": ") {
                key = &sub_str[..i];
                val = &sub_str[i + 2..];
            }
            data.insert(key, val);
        }
        Headers { data }
    }
}

