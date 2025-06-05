use cxx::{type_id, ExternType};
use std::fmt::Debug;

#[allow(dead_code)]
#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("recastnavigation-rs/src/detour_crowd/crowd-ffi.h");

        type dtObstacleAvoidanceParams = crate::detour_crowd::obstacle_avoidance::DtObstacleAvoidanceParams;
    }
}

#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct DtObstacleAvoidanceParams {
    pub vel_bias: f32,
    pub weight_des_vel: f32,
    pub weight_cur_vel: f32,
    pub weight_side: f32,
    pub weight_toi: f32,
    pub horiz_time: f32,
    pub grid_size: u8,
    pub adaptive_divs: u8,
    pub adaptive_rings: u8,
    pub adaptive_depth: u8,
}

unsafe impl ExternType for DtObstacleAvoidanceParams {
    type Id = type_id!("dtObstacleAvoidanceParams");
    type Kind = cxx::kind::Trivial;
}
