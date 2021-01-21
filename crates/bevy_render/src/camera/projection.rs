use super::DepthCalculation;
use bevy_math::Mat4;
use bevy_reflect::{Reflect, ReflectComponent, ReflectDeserialize};
use serde::{Deserialize, Serialize};

pub trait CameraProjection {
    fn get_projection_matrix(&self) -> Mat4;
    fn update(&mut self, width: f32, height: f32);
    fn depth_calculation(&self) -> DepthCalculation;
}

#[derive(Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct PerspectiveProjection {
    pub fov: f32,
    pub aspect_ratio: f32,
    pub near: f32,
    pub far: f32,
}

impl CameraProjection for PerspectiveProjection {
    fn get_projection_matrix(&self) -> Mat4 {
        Mat4::perspective_rh(self.fov, self.aspect_ratio, self.near, self.far)
    }

    fn update(&mut self, width: f32, height: f32) {
        self.aspect_ratio = width / height;
    }

    fn depth_calculation(&self) -> DepthCalculation {
        DepthCalculation::Distance
    }
}

impl Default for PerspectiveProjection {
    fn default() -> Self {
        PerspectiveProjection {
            fov: std::f32::consts::PI / 4.0,
            near: 1.0,
            far: 1000.0,
            aspect_ratio: 1.0,
        }
    }
}

// TODO: make this a component instead of a property
#[derive(Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect_value(Serialize, Deserialize)]
pub enum WindowOrigin {
    Center,
    BottomLeft,
}

#[derive(Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct OrthographicProjection {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
    pub near: f32,
    pub far: f32,
    pub window_origin: WindowOrigin,
}

impl CameraProjection for OrthographicProjection {
    fn get_projection_matrix(&self) -> Mat4 {
        Mat4::orthographic_rh(
            self.left,
            self.right,
            self.bottom,
            self.top,
            self.near,
            self.far,
        )
    }

    fn update(&mut self, width: f32, height: f32) {
        match self.window_origin {
            WindowOrigin::Center => {
                let half_width = width / 2.0;
                let half_height = height / 2.0;
                self.left = -half_width;
                self.right = half_width;
                self.top = half_height;
                self.bottom = -half_height;
            }
            WindowOrigin::BottomLeft => {
                self.left = 0.0;
                self.right = width;
                self.top = height;
                self.bottom = 0.0;
            }
        }
    }

    fn depth_calculation(&self) -> DepthCalculation {
        DepthCalculation::ZDifference
    }
}

impl Default for OrthographicProjection {
    fn default() -> Self {
        OrthographicProjection {
            left: 0.0,
            right: 0.0,
            bottom: 0.0,
            top: 0.0,
            near: 0.0,
            far: 1000.0,
            window_origin: WindowOrigin::Center,
        }
    }
}

#[derive(Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect_value(Serialize, Deserialize)]
pub enum BaseAxis {
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct ScaledOrthographicProjection {
    pub scale: f32,
    pub aspect_ratio: f32,
    pub near: f32,
    pub far: f32,
    pub window_origin: WindowOrigin,
    pub base_axis: BaseAxis,
}

impl CameraProjection for ScaledOrthographicProjection {
    fn get_projection_matrix(&self) -> Mat4 {
        match (&self.window_origin, &self.base_axis) {
            (WindowOrigin::Center, BaseAxis::Vertical) => Mat4::orthographic_rh(
                -self.aspect_ratio * self.scale,
                self.aspect_ratio * self.scale,
                -self.scale,
                self.scale,
                self.near,
                self.far,
            ),
            (WindowOrigin::BottomLeft, BaseAxis::Vertical) => Mat4::orthographic_rh(
                0.0,
                self.aspect_ratio * self.scale,
                0.0,
                self.scale,
                self.near,
                self.far,
            ),
            (WindowOrigin::Center, BaseAxis::Horizontal) => Mat4::orthographic_rh(
                -self.scale,
                self.scale,
                -self.aspect_ratio * self.scale,
                self.aspect_ratio * self.scale,
                self.near,
                self.far,
            ),
            (WindowOrigin::BottomLeft, BaseAxis::Horizontal) => Mat4::orthographic_rh(
                0.0,
                self.scale,
                0.0,
                self.aspect_ratio * self.scale,
                self.near,
                self.far,
            ),
        }
    }

    fn update(&mut self, width: f32, height: f32) {
        self.aspect_ratio = match self.base_axis {
            BaseAxis::Vertical => width / height,
            BaseAxis::Horizontal => height / width,
        }
    }

    fn depth_calculation(&self) -> DepthCalculation {
        DepthCalculation::ZDifference
    }
}

impl Default for ScaledOrthographicProjection {
    fn default() -> Self {
        ScaledOrthographicProjection {
            scale: 1.0,
            aspect_ratio: 1.0,
            near: 0.0,
            far: 1000.0,
            window_origin: WindowOrigin::Center,
            base_axis: BaseAxis::Vertical,
        }
    }
}
