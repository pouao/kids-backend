use deunicode::deunicode_with_tofu;
use mongodb::bson::DateTime;
use regex::Regex;

// Generate friendly slug from the given string
pub async fn slugify(str: &str) -> String {
    let slug = format!(
        "{}-{}",
        deunicode_with_tofu(str.trim(), "-").to_lowercase(),
        DateTime::now().timestamp_millis()
    );

    let re = Regex::new(r"[^a-z-0-9]").unwrap();
    let slug = re.replace_all(&slug, "-");

    let re = Regex::new(r"-{2,}").unwrap();
    let slug = re.replace_all(&slug, "-");

    slug.to_string()
}

// bson::DateTime -> Y-M-D
pub async fn bson_dt_nyr(dt: mongodb::bson::DateTime) -> String {
    dt.to_chrono()
        .with_timezone(
            &chrono::FixedOffset::east_opt(8 * 3600).unwrap(),
        )
        .format(crate::util::constant::DTF_YMD)
        .to_string()
}
