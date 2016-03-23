
use episode;

#[derive(Debug)]
pub struct Subscription {
        pub id: i32,
        pub title: String,
        pub episodes: Option<Vec<Episode>>
}

impl Subscription {
    pub fn episode_count(&self) -> usize { 
        match self.episodes {
            Some(ref e) => e.len(),
            None    => 0,
        }
    }
}
