#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FilterTickets {
    #[serde(rename(serialize = "fromTimeSlot"), with = "standard_format::option")]
    pub from_date: Option<PrimitiveDateTime>,

    #[serde(rename(serialize = "toTimeSlot"), with = "standard_format::option")]
    pub to_date: Option<PrimitiveDateTime>
}

impl FilterTickets {

    pub fn new(from: String, to: String) -> FilterTickets {
        let format = format_description!("[year]-[month]-[day]T[hour]:[minute]");
        let mut filter = FilterTickets {
            from_date: None,
            to_date: None
        };

        if !from.is_empty() {
            if let Ok(date) = PrimitiveDateTime::parse(&from, &format) {
                filter.from_date = Some(date);
            }
        }

        if !to.is_empty() {
            if let Ok(date) = PrimitiveDateTime::parse(&to, &format) {
                filter.to_date = Some(date);
            }
        }

        filter
    }
}

