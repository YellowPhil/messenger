use uuid::Uuid;

pub struct Message {
    id: Uuid,
    from: Uuid,
    content: String
}