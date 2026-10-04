// Automatically generated category module

#[path = "job_sequencing/solution.rs"]
pub mod job_sequencing;

#[path = "minimum_coin_change/solution.rs"]
pub mod minimum_coin_change;

#[path = "smallest_range/solution.rs"]
pub mod smallest_range;

#[path = "stable_matching/solution.rs"]
pub mod stable_matching;


pub use self::job_sequencing::{schedule_jobs, Job, ScheduleResult};
pub use self::minimum_coin_change::find_minimum_change;
pub use self::smallest_range::smallest_range;
pub use self::stable_matching::stable_matching;
