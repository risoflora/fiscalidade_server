#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Payload {
    pub chave_nfe: Option<String>,
}
