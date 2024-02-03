use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ErrorMessageWS {
    pub app_user_id: Uuid,
    pub message: String,
}
