use std::sync::Arc;

use super::topic::{topic_domain::TopicService, topic_repository::TopicRepositoryImpl, topic_service::TopicServiceImpl};

pub struct Injector {
    pub topic_service: Arc<dyn TopicService>
}

impl Injector {
    pub fn new() -> Self {
        let topic_repository = TopicRepositoryImpl::new();
        let topic_service = TopicServiceImpl::new(Box::new(topic_repository));
        Self {
            topic_service: Arc::new(topic_service)
        }
    }
}