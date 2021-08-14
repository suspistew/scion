pub mod topic;

use std::collections::{HashMap, VecDeque};

use serde::{de::DeserializeOwned, ser};
use serde_json::{from_str, to_string};

use crate::core::resources::events::topic::{Topic, TopicConfiguration};

pub type SubscriberId = usize;
pub type Cursor = usize;

pub struct PollConfiguration {
    max_messages: usize,
}

impl Default for PollConfiguration {
    fn default() -> Self { Self { max_messages: 5 } }
}

#[derive(Debug)]
pub enum EventError {
    TopicAlreadyExist,
    TopicDoesNotExist,
    SubscriberIdDoesNotExist,
}

#[derive(Default)]
pub struct Events {
    topics: HashMap<String, Topic>,
    subscribers: HashMap<SubscriberId, (String, PollConfiguration, Cursor)>,
}

impl Events {
    pub fn create_topic(
        &mut self,
        topic_name: &str,
        topic_configuration: TopicConfiguration,
    ) -> Result<(), EventError> {
        if self.topics.contains_key(topic_name) {
            Err(EventError::TopicAlreadyExist)
        } else {
            let topic_string = topic_name.to_string();
            self.topics.insert(topic_string.clone(), Topic::new(topic_string, topic_configuration));
            Ok(())
        }
    }

    pub fn publish<T>(&mut self, topic_name: &str, message: T) -> Result<(), EventError>
    where
        T: ser::Serialize, {
        if !self.topics.contains_key(topic_name) {
            Err(EventError::TopicDoesNotExist)
        } else {
            let message = to_string(&message).unwrap();
            self.topics
                .get_mut(topic_name)
                .expect("A topic is missing, but is identified as existing")
                .publish(message);
            Ok(())
        }
    }

    pub fn subscribe(
        &mut self,
        topic_name: &str,
        poll_configuration: PollConfiguration,
    ) -> Result<SubscriberId, EventError> {
        if !self.topics.contains_key(topic_name) {
            Err(EventError::TopicDoesNotExist)
        } else {
            let next_id = self.subscribers.keys().max().map_or(0, |r| r + 1);
            self.subscribers.insert(
                next_id,
                (
                    topic_name.to_string(),
                    poll_configuration,
                    self.topics
                        .get(topic_name)
                        .expect("A topic is missing, but is identified as existing")
                        .messages
                        .len(),
                ),
            );
            Ok(next_id)
        }
    }

    pub fn poll<T>(&mut self, subscriber_id: &SubscriberId) -> Result<VecDeque<T>, EventError>
    where
        T: DeserializeOwned, {
        if self.subscribers.contains_key(subscriber_id) {
            let (topic_name, poll_configuration, cursor) = self
                .subscribers
                .get_mut(subscriber_id)
                .expect("A topic is missing, but is identified as existing");
            let topic = self
                .topics
                .get(topic_name)
                .expect("A subscriber Id has been linked to a non existing topic");

            let slice_start = *cursor;
            let slice_end = if topic.messages.len() - *cursor < poll_configuration.max_messages {
                topic.messages.len()
            } else {
                *cursor + poll_configuration.max_messages
            };
            let target_slice = &topic.messages[slice_start..slice_end];
            let polled: VecDeque<T> =
                target_slice.iter().filter_map(|message| from_str(message).ok()).collect();
            *cursor += polled.len();
            return Ok(polled);
        }
        Err(EventError::SubscriberIdDoesNotExist)
    }

    pub(crate) fn cleanup(&mut self) {
        self.cleanup_topics_overflow();
        self.cleanup_topics_outdated()
    }

    fn cleanup_topics_outdated(&mut self) {
        let mut min_cursor_for_topics = HashMap::new();
        self.subscribers.values_mut().for_each(|(topic, _, cursor)| {
            let current = min_cursor_for_topics.entry(topic.to_string()).or_insert(*cursor);
            if current > cursor {
                *current = *cursor;
            }
        });

        min_cursor_for_topics.iter().for_each(|(topic, min_cursor)| {
            self.subscribers
                .values_mut()
                .filter(|(t, _, _)| t == topic)
                .for_each(|(_, _, cursor)| *cursor -= *min_cursor);
            self.topics
                .get_mut(topic)
                .expect("A subscriber is referencing a non existing topic")
                .cleanup_outdated(*min_cursor);
        })
    }

    fn cleanup_topics_overflow(&mut self) {
        let mut overflow_counts = HashMap::new();
        self.topics.iter_mut().for_each(|(name, topic)| {
            overflow_counts.insert(name.clone(), topic.cleanup_overflow());
        });

        self.subscribers.iter_mut().for_each(|(_id, subscription)| {
            let overflow = overflow_counts
                .get(&subscription.0)
                .expect("A subscriber is referencing a non existing topic");
            if subscription.2 < *overflow {
                subscription.2 = 0;
            } else {
                subscription.2 -= *overflow;
            }
        });
    }
}

#[cfg(test)]
mod event_tests {
    use crate::core::resources::events::{Events, PollConfiguration, TopicConfiguration};

    #[test]
    fn create_topic_test() {
        let mut event = Events::default();
        assert_eq!(
            true,
            event.create_topic("test_topic", TopicConfiguration { limit: 100 }).is_ok()
        );
        assert_eq!(
            true,
            event.create_topic("test_topic", TopicConfiguration { limit: 90 }).is_err()
        );
    }

    #[test]
    fn event_publish_test() {
        let mut event = Events::default();
        assert_eq!(true, event.publish("test_topic", "Coucou").is_err());
        let _r = event.create_topic("test_topic", TopicConfiguration { limit: 100 });
        assert_eq!(true, event.publish("test_topic", 1).is_ok());

        let topic = event.topics.get("test_topic").expect("topic must be here");
        assert_eq!(1, topic.messages.len());
        assert_eq!(&"1".to_string(), topic.messages.get(0).unwrap());
    }

    #[test]
    fn subscribe_test() {
        let mut event = Events::default();
        assert_eq!(true, event.subscribe("test_topic", PollConfiguration::default()).is_err());

        let _r = event.create_topic("test_topic", TopicConfiguration { limit: 100 });
        let subscribe_result = event.subscribe("test_topic", PollConfiguration::default());
        assert_eq!(true, subscribe_result.is_ok());
        assert_eq!(0, subscribe_result.unwrap());
        let subscribe_result2 = event.subscribe("test_topic", PollConfiguration::default());
        assert_eq!(true, subscribe_result2.is_ok());
        assert_eq!(1, subscribe_result2.unwrap());
    }

    #[test]
    fn poll_test() {
        let mut event = Events::default();
        let _r = event.create_topic("test_topic", TopicConfiguration { limit: 100 });
        let subscriber_id = event.subscribe("test_topic", PollConfiguration::default()).unwrap();
        let _r = event.publish("test_topic", 42);

        let mut poll_result = event.poll::<usize>(&subscriber_id).unwrap();
        assert_eq!(1, poll_result.len());
        assert_eq!(42, poll_result.pop_front().unwrap());

        let _r = event.publish("test_topic", 4);
        let _r = event.publish("test_topic", 8);
        let _r = event.publish("test_topic", 12);
        let _r = event.publish("test_topic", 16);
        let _r = event.publish("test_topic", 20);
        let _r = event.publish("test_topic", 24);
        let mut poll_result = event.poll::<usize>(&subscriber_id).unwrap();
        assert_eq!(5, poll_result.len());
        assert_eq!(4, poll_result.pop_front().unwrap());
        assert_eq!(8, poll_result.pop_front().unwrap());
        assert_eq!(12, poll_result.pop_front().unwrap());
    }

    #[test]
    fn cleanup_test() {
        let mut event = Events::default();
        let _r = event.create_topic("test_topic", TopicConfiguration { limit: 3 });
        let subscriber_id =
            event.subscribe("test_topic", PollConfiguration { max_messages: 2 }).unwrap();
        let subscriber_id2 =
            event.subscribe("test_topic", PollConfiguration { max_messages: 1 }).unwrap();

        let _r = event.publish("test_topic", 4);
        let _r = event.publish("test_topic", 8);
        let _r = event.publish("test_topic", 12);
        let _r = event.publish("test_topic", 16);

        assert_eq!(4, event.topics.get("test_topic").unwrap().messages.len());
        event.cleanup();
        assert_eq!(3, event.topics.get("test_topic").unwrap().messages.len());
        let poll_result = event.poll::<usize>(&subscriber_id).unwrap();
        let poll_result2 = event.poll::<usize>(&subscriber_id2).unwrap();
        assert_eq!(2, poll_result.len());
        assert_eq!(1, poll_result2.len());
        assert_eq!(3, event.topics.get("test_topic").unwrap().messages.len());
        event.cleanup();
        assert_eq!(2, event.topics.get("test_topic").unwrap().messages.len());
    }
}
