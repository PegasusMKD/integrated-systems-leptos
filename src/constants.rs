use time::format_description::FormatItem;

pub const DATE_TIME_FORMAT: &[FormatItem<'_>] = time::macros::format_description!(
    "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond]"
);

static_toml::static_toml! {
    pub static CONFIG = include_toml!("deployment.toml");
}
