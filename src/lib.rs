use std::os::raw::c_double;

use rivets::detour;
use tracing::info;

#[repr(C)] struct MapPosition;
#[repr(C)] struct ForceID;
#[repr(C)] struct EntityWithHealth;

#[detour(?findRandomTarget@Surface@@QEAAPEAVEntityWithHealth@@VMapPosition@@VForceID@@NAEBV?$function@$$A6A_NAEBVEntityWithHealth@@@Z@std@@@Z)]
extern "C" fn find_random_target(_: MapPosition, _: ForceID, _: c_double, _: ()) -> *const EntityWithHealth {
    info!("Detoured into Surface::findRandomTarget!");
    1 as *const EntityWithHealth
}
