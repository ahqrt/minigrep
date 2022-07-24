pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let size = args.len();

        let case_sensitive = if size == 4 {
            args[3].clone()
        } else {
            "0".to_string()
        };

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}
