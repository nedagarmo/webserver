use std::collections::HashMap;
use std::str::SplitN;

#[derive(Debug)]
pub struct QueryString<'buffer> {
    parameters: HashMap<&'buffer str, Value<'buffer>>,
}

#[derive(Debug)]
enum Value<'buffer> {
    Single(&'buffer str),
    Multiple(Vec<&'buffer str>),
}

impl<'buffer> QueryString<'buffer> {
    pub fn get(&self, key: &str) -> Option<&Value<'buffer>> {
        self.parameters.get(key)
    }
}

impl<'buffer> From<&'buffer str> for QueryString<'buffer> {
    fn from(s: &'buffer str) -> Self {
        let mut parameters = HashMap::new();
        for mut parameter in s.split('&') {
            parameter = parameter.trim();
            if !parameter.is_empty() {
                let mut key = parameter;
                let mut value = "";
                if let Some(i) = parameter.find('=') {
                    key = &parameter[..i];
                    value = &parameter[i + 1..];
                }

                parameters.entry(key).and_modify(|entry_value| match entry_value {
                    Value::Single(current_value) => {
                        *entry_value = Value::Multiple(vec![current_value, value]);
                    },
                    Value::Multiple(entry_value) => entry_value.push(value),
                }).or_insert(Value::Single(value));
            }
        }

        QueryString { parameters }
    }
}