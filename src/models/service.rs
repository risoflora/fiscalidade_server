use chrono::NaiveDateTime;

#[derive(Serialize, Queryable)]
pub struct QueryableService {
    pub id: i64,
    pub description: String,
    pub slug: String,
    pub active: bool,
    pub created_at: NaiveDateTime,
}
