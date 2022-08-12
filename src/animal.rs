//! The example traits

use std::convert::From;
use std::time::Duration;

/// Structs that implement the `Animal` trait represent an animal
pub trait Animal {
    /// The species of the animal
    const SPECIES: &'static str;
    /// The sound the animal makes
    const SOUND: &'static str;

    /// How to get the animal's name
    fn name(&self) -> &str;

    /// Makes the animal make a sound in the console
    fn speak(&self) {
        println!(
            "{} the {} goes '{}'.",
            &self.name(),
            Self::SPECIES,
            Self::SOUND
        );
    }
}

fn round_n_dp<FloatType>(n: FloatType, dp: u32) -> f64
where
    f64: From<FloatType> {
    let power = 10u8.pow(dp) as f64;
    (f64::from(n) * power).round() / power
}

/// Structs that implement the `Walks` trait represent an [`Animal`] that can walk
pub trait Walks
where
    Self: Animal,
{
    /// The speed of the animal in m/s
    const WALKING_SPEED: f32;

    /// How to get the speed modifier of this individual animal - `speed = WALKING_SPEED * animal.speed_modifier()`. The default implementation always returns 1
    fn speed_modifier(&self) -> f32 {
        1f32
    }

    /// Make the animal walk for a given time and returns the distance travelled (in metres)
    fn walk_time(&self, time: Duration) -> f64 {
        let distance = Self::WALKING_SPEED as f64 * self.speed_modifier() as f64 * time.as_secs_f64();
        println!(
            "{} takes {}s to walk {}m",
            &self.name(),
            round_n_dp(time.as_secs_f64(), 2),
            round_n_dp(distance, 2)
        );
        distance as f64
    }

    /// Make the animal walk a given distance (in metres) and returns the time taken
    fn walk_distance<T>(&self, distance: T) -> Duration
    where
        f64: From<T>
    {
        let distance = f64::from(distance);
        let time_secs = distance / (Self::WALKING_SPEED as f64 * self.speed_modifier() as f64);
        println!(
            "{} takes {}s to walk {}m",
            &self.name(),
            round_n_dp::<f64>(time_secs, 2),
            round_n_dp::<f64>(distance, 2)
        );
        Duration::from_secs_f64(time_secs)
    }
}
