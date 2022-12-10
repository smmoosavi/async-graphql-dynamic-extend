use regex::Regex;

#[cfg(test)]
pub fn normalize_schema(sdl: &str) -> String {
    let re = Regex::new(r"\s+").unwrap();
    re.replace_all(sdl, " ").trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_schema() {
        let sdl = "
            type Query {
                hello: String!
            }
            schema {
                query: Query
            }
        ";
        assert_eq!(
            normalize_schema(sdl),
            "type Query { hello: String! } schema { query: Query }"
        );
    }
}
