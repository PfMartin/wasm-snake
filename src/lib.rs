use wee_alloc::WeeAlloc;

pub mod snake;
pub mod world;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;
