#![allow(unused)]
use autocxx::{prelude::*, subclass::CppSubclassSelfOwnedDefault};
use std::pin::{pin, Pin};

include_cpp! {
    #include "GamepadMotion.hpp"
    generate!("GamepadMotion")
    generate!("GamepadMotionSettings")
    generate!("GamepadMotionHelpers::GyroCalibration")
    generate!("GamepadMotionHelpers::AutoCalibration")
    generate!("GamepadMotionHelpers::Motion")
    safety!(unsafe_ffi)
}

type CalibrationMode = ffi::GamepadMotionHelpers::CalibrationMode;

pub struct GamepadMotion(UniquePtr<ffi::GamepadMotion>);

impl GamepadMotion {
    #[inline]
    pub fn new() -> Self {
        Self(ffi::GamepadMotion::new().within_unique_ptr())
    }

    #[inline]
    pub fn gyro_player_space(&mut self, yaw_relax_factor: Option<f32>) -> [f32; 2] {
        let mut xy = [0f32; 2];
        self.0.pin_mut().GetPlayerSpaceGyro(
            pin!(xy[0]),
            pin!(xy[1]),
            yaw_relax_factor.unwrap_or(1.41),
        );
        xy
    }

    #[inline]
    pub fn gyro_calibrated(&mut self) -> [f32; 3] {
        let mut xyz = [0f32; 3];
        self.0
            .pin_mut()
            .GetCalibratedGyro(pin!(xyz[0]), pin!(xyz[1]), pin!(xyz[2]));
        xyz
    }

    #[inline]
    pub fn gyro_world_space(&mut self, side_reduc_thresh: Option<f32>) -> [f32; 2] {
        let mut xy = [0f32; 2];
        self.0.pin_mut().GetWorldSpaceGyro(
            pin!(xy[0]),
            pin!(xy[1]),
            side_reduc_thresh.unwrap_or(0.125),
        );
        xy
    }

    #[inline]
    pub fn process(&mut self, g: &[f32; 3], a: &[f32; 3], dt: &f32) -> &Self {
        self.0
            .pin_mut()
            .ProcessMotion(g[0], g[1], g[2], a[0], a[1], a[2], *dt);
        self
    }

    #[inline]
    pub fn calib_mode_set(&mut self, c: CalibrationMode) {
        self.0.pin_mut().SetCalibrationMode(c);
    }

    #[inline]
    pub fn calib_mode(&mut self) -> CalibrationMode {
        self.0.pin_mut().GetCalibrationMode()
    }

    #[inline]
    pub fn calib_offset_set(&mut self, xyz: [f32; 3], w: i32) {
        self.0
            .pin_mut()
            .SetCalibrationOffset(xyz[0], xyz[1], xyz[2], w.into());
    }

    #[inline]
    pub fn calib_offset(&mut self) -> [f32; 3] {
        let mut xyz = [0f32; 3];
        self.0
            .pin_mut()
            .GetCalibrationOffset(pin!(xyz[0]), pin!(xyz[1]), pin!(xyz[2]));
        xyz
    }

    #[inline]
    pub fn orientation(&mut self) -> [f32; 4] {
        let mut xyzw = [0f32; 4];
        self.0
            .pin_mut()
            .GetOrientation(pin!(xyzw[3]), pin!(xyzw[0]), pin!(xyzw[1]), pin!(xyzw[2]));
        xyzw
    }

    #[inline]
    pub fn gravity(&mut self) -> [f32; 3] {
        let mut xyz = [0f32; 3];
        self.0
            .pin_mut()
            .GetGravity(pin!(xyz[0]), pin!(xyz[1]), pin!(xyz[2]));
        xyz
    }

    #[inline]
    pub fn accel_processed(&mut self) -> [f32; 3] {
        let mut xyz = [0f32; 3];
        self.0
            .pin_mut()
            .GetProcessedAcceleration(pin!(xyz[0]), pin!(xyz[1]), pin!(xyz[2]));
        xyz
    }

    #[inline]
    pub fn calib_cont_start(&mut self) {
        self.0.pin_mut().StartContinuousCalibration();
    }

    #[inline]
    pub fn calib_cont_pause(&mut self) {
        self.0.pin_mut().PauseContinuousCalibration();
    }

    #[inline]
    pub fn calib_cont_reset(&mut self) {
        self.0.pin_mut().ResetContinuousCalibration();
    }

    #[inline]
    pub fn calib_auto_steady(&mut self) -> bool {
        self.0.pin_mut().GetAutoCalibrationIsSteady()
    }

    #[inline]
    pub fn calib_auto_confid(&mut self) -> f32 {
        self.0.pin_mut().GetAutoCalibrationConfidence()
    }

    #[inline]
    pub fn calib_auto_confid_set(&mut self, confid: f32) {
        self.0.pin_mut().SetAutoCalibrationConfidence(confid);
    }

    #[inline]
    pub fn reset(&mut self) {
        self.0.pin_mut().Reset();
    }

    #[inline]
    pub fn reset_motion(&mut self) {
        self.0.pin_mut().ResetMotion();
    }
}

struct Settings(UniquePtr<ffi::GamepadMotionSettings>);

impl Settings {
    #[inline]
    pub fn new() -> Self {
        Self(ffi::GamepadMotionSettings::new().within_unique_ptr())
    }
}

struct AutoCalibration(UniquePtr<ffi::GamepadMotionHelpers::AutoCalibration>);

impl AutoCalibration {
    #[inline]
    pub fn new() -> Self {
        Self(ffi::GamepadMotionHelpers::AutoCalibration::new().within_unique_ptr())
    }

    #[inline]
    pub fn steady(&mut self) -> bool {
        self.0.pin_mut().IsSteady()
    }

    #[inline]
    pub fn reset(&mut self) {
        self.0.pin_mut().Reset();
    }
}

struct GyroCalibration(UniquePtr<ffi::GamepadMotionHelpers::GyroCalibration>);

struct Motion(UniquePtr<ffi::GamepadMotionHelpers::Motion>);

struct Vec(UniquePtr<ffi::GamepadMotionHelpers::Vec>);

#[cfg(test)]
mod tests {
    use super::*;
}
