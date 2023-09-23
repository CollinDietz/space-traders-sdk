#[macro_export]
macro_rules! string {
    ($s:expr) => {
        String::from($s)
    };
}

pub struct _OrderedJsonObject {
    fields: Vec<(String, String)>,
}

impl _OrderedJsonObject {
    pub fn _new() -> Self {
        _OrderedJsonObject { fields: Vec::new() }
    }

    pub fn _add_field(&mut self, key: &str, value: String) {
        self.fields.push((key.to_owned(), value));
    }

    pub fn _to_json_string(&self) -> String {
        let mut json = String::from("{");
        for (i, (key, value)) in self.fields.iter().enumerate() {
            json.push_str(format!(r#""{}":"{}""#, key, value).as_str());
            if i < self.fields.len() - 1 {
                json.push(',');
            }
        }
        json.push('}');
        json
    }
}

#[macro_export]
macro_rules! ordered_json {
    ($($key:expr => $value:expr),*) => {{
        let mut ordered_obj = crate::utils::_OrderedJsonObject::_new();
        $(ordered_obj._add_field($key, $value.to_string());)*
        ordered_obj._to_json_string()
    }};
}
