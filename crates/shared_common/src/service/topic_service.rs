pub trait TopicService {
    fn send_message(&self, message: String) -> Result<(), String>;
}
