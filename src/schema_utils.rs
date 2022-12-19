use regex::Regex;

pub fn normalize_schema(sdl: &str) -> String {
    let sdl = Regex::new(r"\s+")
        .unwrap()
        .replace_all(sdl, " ")
        .trim()
        .to_string();
    let sdl = Regex::new(r"\s*\{\s*")
        .unwrap()
        .replace_all(&sdl, " {\n")
        .to_string();
    let sdl = Regex::new(r"\s+(\S+\s*:\s*\S+)")
        .unwrap()
        .replace_all(&sdl, "\n  $1")
        .to_string();
    let sdl = Regex::new(r"\s*}\s*")
        .unwrap()
        .replace_all(&sdl, "\n}\n")
        .to_string();

    sdl
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_normalize_schema() {
        let sdl = "


            type Query {
                hello: String!
                nice: String!
                bye: String!
            }

            schema { query: Query }
        ";
        assert_eq!(
            normalize_schema(sdl),
            indoc! {"
                type Query {
                  hello: String!
                  nice: String!
                  bye: String!
                }
                schema {
                  query: Query
                }

"}
        );
    }
}
