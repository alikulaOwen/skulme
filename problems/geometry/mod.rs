// Automatically generated category module

#[path = "closest_points/solution.rs"]
pub mod closest_points;

#[path = "graham_scan/solution.rs"]
pub mod graham_scan;

#[path = "jarvis_scan/solution.rs"]
pub mod jarvis_scan;

#[path = "point/solution.rs"]
pub mod point;

#[path = "polygon_points/solution.rs"]
pub mod polygon_points;

#[path = "ramer_douglas_peucker/solution.rs"]
pub mod ramer_douglas_peucker;

#[path = "segment/solution.rs"]
pub mod segment;


pub use self::closest_points::closest_points;
pub use self::graham_scan::graham_scan;
pub use self::jarvis_scan::jarvis_march;
pub use self::point::Point;
pub use self::polygon_points::lattice_points;
pub use self::ramer_douglas_peucker::ramer_douglas_peucker;
pub use self::segment::Segment;
