extern crate rusqlite;
extern crate time;

use std::sync::Arc;
use std::sync::Mutex;

use std::path::Path;
use rusqlite::Connection;
use std::collections::HashMap;
use std::collections::HashSet;

use super::types::Subscription;
use super::types::Episode;
use super::types::SubscriptionKey;
use super::types::EpisodeKey;
use super::types::AMEpisode;
use super::types::AMSubscription;

enum StateError { InvalidState }

pub struct Library {
    subscriptions:      Vec<AMSubscription>,
    subscriptions_map:  HashMap<SubscriptionKey, AMSubscription>,
    episodes:           Vec<AMEpisode>,
    episodes_map:       HashMap<EpisodeKey, AMEpisode>,
    episode_ids:        HashSet<EpisodeKey>    
}

impl Library {

    pub fn new() -> Self {
        Library {
            subscriptions:      Vec::new(),
            subscriptions_map:  HashMap::new(),
            episodes:           Vec::new(),
            episodes_map:       HashMap::new(),
            episode_ids:        HashSet::new()
        }
    }

    pub fn add_subscription(&mut self, item: AMSubscription) {
        let id = item.lock().unwrap().get_id();

        self.subscriptions_map.insert(id, item.clone());        
        self.subscriptions.push(item);
    }

    pub fn add_episode(&mut self, episode: AMEpisode) {
        let id = episode.get_id();

        if self.episode_ids.contains(&id) {
            return;
        }

        self.episode_ids.insert(id);
        self.episodes.push(episode.clone());
        self.episodes_map.insert(id, episode.clone());

        let subm = episode.get_subscription();

        let sub = subm.lock().unwrap();
        let key = sub.get_key();
        match self.subscriptions_map.get_mut(&key) {
            Some(subscription) => subscription.lock().unwrap().add_episode(episode.clone()),
            None               => println!("{} doens't match any subscription", key)
        }
    }

    pub fn remove_subscription(&mut self, item: AMSubscription) {
        let id = item.lock().unwrap().get_key();

        let index = self.subscriptions.iter().position(|x| x.lock().unwrap().get_key() == item.lock().unwrap().get_key()).unwrap();
        self.subscriptions.remove(index);
        self.subscriptions_map.remove(&id);

    }

    pub fn clear_subscriptions(&mut self) {
        self.subscriptions_map.clear();
        self.subscriptions.clear();
    }

    pub fn get_subscription(&mut self, id: SubscriptionKey) -> Option<&AMSubscription> {
        return self.subscriptions_map.get(&id);
    }

    pub fn get_episodes(&mut self) -> &Vec<AMEpisode> {
        return &self.episodes;
    }

    pub fn get_episode(&mut self, id: EpisodeKey) -> Option<&AMEpisode> {
        return self.episodes_map.get(&id); 
    }

    fn load_subscriptions() {
        let path = Path::new("/home/bottiger/Rust/SoundWaves-Core/test_data/podcast.db");
        let conno = Connection::open(path);

        match conno {
            Ok(ref v) => println!("Database opened: {:?}", v),
            Err(ref e) => println!("Database could not be opened: {:?}", e),
        }

        let conn = conno.unwrap();
        let mut stmt = conn.prepare("SELECT _id, title FROM subscriptions").unwrap();
        let person_iter = stmt.query_map(&[], |row| {
            Subscription {
                id: row.get(0),
                title: row.get(1),
                episodes: Vec::new()
            }
        }).unwrap();

        for person in person_iter {
            let sub = person.unwrap();
            let count = sub.episode_count();
            println!("Found person {:?}, episodes: {:?}", sub, count);
        }
}


}
