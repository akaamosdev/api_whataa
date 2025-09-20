

#[derive(Debug,Serialize,Deserialize,FromRow)]
pub struct Admin{
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub phone: String,
    pub compagny_id: String,
    pub synchronise: bool
}

