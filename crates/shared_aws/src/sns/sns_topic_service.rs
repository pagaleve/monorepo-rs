use shared_common::service::topic_service::TopicService;

pub struct SnsTopicService {}

impl TopicService for SnsTopicService {
    // This is a dummy implementation but in the future we will implement the real logic using AWS SNS
    fn send_message(&self, message: String) -> Result<(), String> {
        Ok(())
    }
}
