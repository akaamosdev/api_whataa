use serde::Deserialize;


#[derive(Deserialize)]
pub struct PaginateParam {
    pub offset: Option<i64>,
    pub search: Option<String>,
    pub depot_id: Option<String>,
}

#[derive(Deserialize)]
pub struct PaginateDocument {
    pub offset: Option<i64>,
    pub search: Option<String>,
    pub type_doc: i8,
}

#[derive(Deserialize)]
pub struct PaginateReglement {
    pub offset: Option<i64>,
    pub search: Option<String>,
}


