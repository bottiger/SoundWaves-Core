
extern crate time;

use time::Timespec;
use std::sync::Arc;
use std::sync::Mutex;

pub type AMSubscription = Arc<Mutex<Subscription>>;
pub type AMEpisode      = Arc<Mutex<Episode>>;

pub type SubscriptionKey = i32;
pub type EpisodeKey = i32;

#[derive(Clone, Debug)]
pub struct Subscription {
        pub id: i32,
        pub title: String,
        pub episodes: Vec<Arc<Mutex<Episode>>>
}

impl Subscription {

    pub fn new() -> Self {
        Subscription {
            id:         -1,
            title:       "".to_owned(),
            episodes:    Vec::new()
        }
    }

    pub fn get_id(&self) -> i32 {
        return self.id;
    }

    pub fn get_key(&self) -> SubscriptionKey {
        return self.id;
    }
    
    pub fn episode_count(&self) -> usize { 
        return self.episodes.len();
    }

    pub fn add_episode(&mut self, item: Arc<Mutex<Episode>>) {
        self.episodes.push(item);
    }
}

#[derive(Clone, Debug)]
pub struct Episode {
        id:            i32,
        url:           Option<String>,
        title:         Option<String>,
        author:        Option<String>,
        date:          Option<Timespec>,
        pub_date:      Option<Timespec>,
        content:       Option<String>,
        duration:      Option<i64>,
        image:         Option<String>,
        filesize:      Option<i64>,
        filename:      Option<String>,
        is_downloaded: Option<bool>,
        offset:        Option<i32>,
        status:        Option<i32>,
        listened:      Option<bool>,
        priority:      Option<i32>,
        length:        Option<i64>,
        last_update:   Option<i64>,
        sub_title:     Option<String>,
        created_at:    Option<Timespec>,
        subscription: Arc<Mutex<Subscription>>
}

impl Episode {

    pub fn new(sub: Arc<Mutex<Subscription>>) -> Self {
        Episode {
            id:             -1,
            url:            None,
            title:          None,
            author:         None,
            date:           None,
            pub_date:       None,
            content:        None,
            duration:       None,
            image:          None,
            filesize:       None,
            filename:       None,
            is_downloaded:  None,
            offset:         None,
            status:         None,
            listened:       None,
            priority:       None,
            length:         None,
            last_update:    None,
            sub_title:      None,
            created_at:     None,
            subscription:   sub
        }
    }

    pub fn get_id(&self) -> i32 {
        return self.id;
    }

    pub fn set_subscription(&mut self, subscription: Arc<Mutex<Subscription>>) {
        self.subscription = subscription;
    }

    pub fn get_subscription(&self) -> Arc<Mutex<Subscription>> {
        return self.subscription.clone();
    }

    pub fn notify_property_changed(&self) {

    }
}
