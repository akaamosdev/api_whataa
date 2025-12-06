use serde::Deserialize;


#[derive(Deserialize)]
pub struct PaginateParam {
    pub offset: Option<i64>,
    pub search: Option<String>,
    pub depot_id: Option<String>,
    pub type_tier: Option<String>,
}

#[derive(Deserialize)]
pub struct PaginateDocument {
    pub offset: i32,
    pub search: Option<String>,
    pub type_doc: i32,
    pub type_tier: String,
    pub limit: i64,
    pub date_start: Option<String>,
    pub date_end: Option<String>,

}

#[derive(Deserialize)]
pub struct PaginateReglement {
    pub offset: Option<i64>,
    pub search: Option<String>,
}


