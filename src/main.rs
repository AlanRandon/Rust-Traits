//! An example of using traits to implement animal structs

use animal::{Animal, Walks};
use std::borrow::Cow;
use std::time::Duration;

mod animal;

struct Dog<'a> {
    name: Cow<'a, str>,
    speed_modifier: f32
}

impl Animal for Dog<'_> {
    const SPECIES: &'static str = "dog";
    const SOUND: &'static str = "woof";

    fn name(&self) -> &str {
        &self.name
    }
}

impl Walks for Dog<'_> {
    const WALKING_SPEED: f32 = 0.447;

    fn speed_modifier(&self) -> f32 {
        self.speed_modifier
    }
}

struct Goat<'a> {
    name: Cow<'a, str>,
    speed_modifier: f32
}

impl Animal for Goat<'_> {
    const SPECIES: &'static str = "sheep";
    const SOUND: &'static str = "baa";

    fn name(&self) -> &str {
        &self.name
    }
}

impl Walks for Goat<'_> {
    const WALKING_SPEED: f32 = 4.47;

    fn speed_modifier(&self) -> f32 {
        self.speed_modifier
    }
}

fn main() {
    let dog = Dog {
        name: Cow::from("Lundy"),
        speed_modifier: 2f32
    };
    dog.speak();
  dog.walk_time(Duration::from_secs_f64(10f64));
}
