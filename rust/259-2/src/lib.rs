use std::collections::HashMap;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum ParsedValue {
    String(String),
    Number(i64),
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Parsed {
    name: String,
    values: HashMap<String, ParsedValue>
}


pub fn parse_line(line: &str) -> Parsed {
    let mut name = String::new();
    let mut values: HashMap<String, ParsedValue> = HashMap::new();
    let mut line_elems = line.split_whitespace();
    line_elems.next();
    name.push_str(line_elems.next().unwrap());

    for elem in line_elems {
        let mut key_value = elem.split('=');
        let key = match key_value.next() {
            None => continue,
            Some(k) => k.to_string(),
        };
        let value = match key_value.next() {
            None => continue,
            Some(v) => v,
        };
        let value = match value.parse::<i64>() {
            Ok(v) => ParsedValue::Number(v),
            Err(_) => {
                let strlen = value.len() - 1;
                let v = &value[1..strlen];
                ParsedValue::String(v.to_string())
            }
        };
        values.insert(key, value);
    }

    Parsed {
        name,
        values 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let line = "{%  id   field1=\"value1\"    field2=\"value2\"  field3=42 %}";
        let mut values: HashMap<String, ParsedValue> = HashMap::new();
        values.insert("field1".to_string(), ParsedValue::String("value1".to_string()));
        values.insert("field2".to_string(), ParsedValue::String("value2".to_string()));
        values.insert("field3".to_string(), ParsedValue::Number(42));
        let result = Parsed {
            name: "id".to_string(),
            values
        };
        assert_eq!(parse_line(&line), result);
    }

    #[test]
    fn test2() {
        let line = "{%  youtube title=\"Title \\\"quoted\\\" done\" %}";
        let mut values: HashMap<String, ParsedValue> = HashMap::new();
        values.insert("title".to_string(), ParsedValue::String("Title \\\"quoted\\\" done".to_string()));
        let result = Parsed {
            name: "youtube".to_string(),
            values
        };
        assert_eq!(parse_line(&line), result);
    }

    #[test]
    fn test3() {
        let line = "{%  youtube title=\"Title with escaped backslash \\\\\" %}";
        let mut values: HashMap<String, ParsedValue> = HashMap::new();
        values.insert("title".to_string(), ParsedValue::String("Title with escaped backslash \\\\".to_string()));
        let result = Parsed {
            name: "youtube".to_string(),
            values
        };
        assert_eq!(parse_line(&line), result);
    }
}
