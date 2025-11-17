pub struct Message {
    pub destination: uuid::Uuid,
    pub payload: bytes::Bytes,
}
