use super::Uuid;

#[derive(Default, Clone, PartialEq)]
pub enum CaseStatus {
    #[default]
    Closed = 0,
    Open = 1,
}

pub struct CreateCaseInput {}
pub struct UpdateCaseInput {}

#[derive(Default, Clone)]
pub struct Case {
    pub id: Uuid,
    pub name: String,
    pub url: String,
    pub status: CaseStatus,
}
