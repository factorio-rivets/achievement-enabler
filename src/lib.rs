use std::os::raw::c_double;

use rivets::detour;
use tracing::info;

#[repr(C)] struct MapPosition;
#[repr(C)] struct ForceID;
#[repr(C)] struct Entity;

#[detour(_ZN7Surface16findRandomTargetE11MapPosition7ForceIDdRKSt8functionIFbRK16EntityWithHealthEE)]
fn find_random_target(_: MapPosition, _: ForceID, _: c_double, _: ()) -> *const Entity {
    info!("Detoured into Surface::findRandomTarget!");
    1 as *const Entity
}
