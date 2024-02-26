use crate::{models::topic::topic_domain::
    {
        Topic, TopicRepository, TopicService
    }, 
    utils::callback::ResponseCallback
};

pub struct TopicServiceImpl {
    pub repository: Box<dyn TopicRepository>,
}

impl TopicService for TopicServiceImpl {
    fn save(&self, topic: Topic, callback: ResponseCallback<(), String>) {
        self.repository.save(topic, callback);
    }
}

impl TopicServiceImpl {
    pub fn new(repository: Box<dyn TopicRepository>) -> Self {
        Self { repository }
    }
}