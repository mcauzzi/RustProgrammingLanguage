#[cfg(test)]
mod tests{
    use std::cell::RefCell;

    use crate::messenger::{LimitTracker, Messenger};

    struct MockMessenger{
        sent_messages:RefCell<Vec<String>>,
    }
    impl MockMessenger{
        fn new()->MockMessenger{
            MockMessenger{
                sent_messages:RefCell::new(vec![])
            }
        }
    }

    impl Messenger for MockMessenger{
        fn send(&self, msg:&str){
            self.sent_messages.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning(){
        let mock_messenger=MockMessenger::new();

        let mut limit_tracker=LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}