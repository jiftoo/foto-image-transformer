mod tests {
    use std::env;

    use json_minimal::Json;

    macro_rules! cast {
        ($target: expr, $pat: path) => {{
            if let $pat(a) = $target {
                a
            } else {
                panic!("mismatch variant when cast to {}", stringify!($pat));
            }
        }};
    }

    fn parse_commands() -> Json {
        let out = std::process::Command::new("node")
            .arg("data/jsoparse.js")
            .arg("data/commands.js")
            .output()
            .expect("failed");
        return Json::parse(&out.stdout).unwrap();
    }

    #[test]
    fn test() {
        #[path = "../../src/proc.rs"]
        mod proc;

        let json = cast!(parse_commands(), Json::ARRAY);
        for a in json {
            match a.get("name").unwrap().unbox() {
                Json::JSON(vals) => {
                    println!("{}", vals.len());
                }
                Json::OBJECT { name, value } => match value {
                    Json::STRING(val) => {
                        println!("{}:{}", name, val);
                    }
                },
                _ => {}
            }
        }
        // proc::execute(vec![]);
    }
}
