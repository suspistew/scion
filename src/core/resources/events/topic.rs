pub(crate) struct Topic {
    _name: String,
    configuration: TopicConfiguration,
    pub(crate) messages: Vec<String>,
}

impl Topic {
    pub(crate) fn new(name: String, configuration: TopicConfiguration) -> Self {
        Self {
            _name: name.to_string(),
            messages: Vec::with_capacity(configuration.limit),
            configuration,
        }
    }

    pub(crate) fn publish(&mut self, message: String) {
        self.messages.push(message);
    }

    pub(crate) fn cleanup_overflow(&mut self) -> usize {
        if self.messages.len() > self.configuration.limit {
            let overflow = self.messages.len() - self.configuration.limit;
            self.messages = self.messages
                [self.messages.len() - self.configuration.limit..self.messages.len()]
                .to_vec();
            overflow
        } else {
            0
        }
    }

    pub(crate) fn cleanup_outdated(&mut self, min_index: usize) {
        if min_index > 0 {
            self.messages = self.messages[min_index..self.messages.len()].to_vec();
        }
    }
}

#[derive(Clone)]
pub struct TopicConfiguration {
    pub limit: usize,
}

impl Default for TopicConfiguration {
    fn default() -> Self {
        Self { limit: 50 }
    }
}

#[cfg(test)]
mod topic_tests {
    use super::{Topic, TopicConfiguration};

    #[test]
    fn topic_cleanup_overflow_test() {
        let mut topic = Topic::new("test".to_string(), TopicConfiguration { limit: 3 });
        for _i in 0..4 {
            topic.publish("1".to_string());
        }

        assert_eq!(1, topic.cleanup_overflow());

        for _i in 0..40 {
            topic.publish("1".to_string());
        }
        assert_eq!(43, topic.messages.len());
        assert_eq!(40, topic.cleanup_overflow());
        assert_eq!(3, topic.messages.len());
    }

    #[test]
    fn topic_cleanup_outdated_test() {
        let mut topic = Topic::new("test".to_string(), TopicConfiguration { limit: 3 });
        for _i in 0..3 {
            topic.publish("1".to_string());
        }
        topic.cleanup_outdated(1);
        assert_eq!(2, topic.messages.len());
    }
}
