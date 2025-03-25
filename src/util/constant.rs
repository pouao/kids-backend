use lazy_static::lazy_static;
use std::collections::HashMap;

// async-graphql result type
pub type GqlResult<T> = std::result::Result<T, async_graphql::Error>;

// datetime format
pub const DTF_YMD: &str = "%Y-%m-%d";
pub const DTF_YMDHMSZ: &str = "%Y-%m-%d %H:%M:%S%Z";

lazy_static! {
    // CFG variables defined in .env file
    pub static ref CFG: HashMap<&'static str, String> = {
        dotenvy::dotenv().ok();

        let mut map = HashMap::new();

        map.insert(
            "ADDR",
            dotenvy::var("ADDR").expect("Expected ADDR to be set in env!"),
        );
        map.insert(
            "PORT",
            dotenvy::var("PORT").expect("Expected PORT to be set in env!"),
        );
        map.insert(
            "GQL_URI",
            dotenvy::var("GQL_URI").expect("Expected GQL_URI to be set in env!"),
        );

        map.insert(
            "SITE_KID",
            dotenvy::var("SITE_KID").expect("Expected SITE_KID to be set in env!"),
        );
        map.insert(
            "SITE_KEY",
            dotenvy::var("SITE_KEY").expect("Expected SITE_KEY to be set in env!"),
        );
        map.insert(
            "CLAIM_EXP",
            dotenvy::var("CLAIM_EXP").expect("Expected CLAIM_EXP to be set in env!"),
        );

        map.insert(
            "MONGODB_URI",
            dotenvy::var("MONGODB_URI").expect("Expected MONGODB_URI to be set in env!"),
        );
        map.insert(
            "MONGODB_NAME",
            dotenvy::var("MONGODB_NAME").expect("Expected MONGODB_NAME to be set in env!"),
        );
        map.insert(
            "PAGE_SIZE",
            dotenvy::var("PAGE_SIZE").expect("Expected PAGE_SIZE to be set in env!"),
        );

        map
    };
}
