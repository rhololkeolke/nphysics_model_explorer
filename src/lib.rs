use nalgebra as na;

#[derive(Debug, Clone, PartialEq)]
pub struct ColliderUserData<N>
where
    N: na::RealField,
{
    pub rgba: Option<na::Point4<f32>>,
    pub torsional_friction: N,
    pub rolling_friction: N,
}

impl<N> Default for ColliderUserData<N>
where
    N: na::RealField + From<f32>,
{
    fn default() -> Self {
        ColliderUserData {
            rgba: None,
            torsional_friction: N::from(0.005),
            rolling_friction: N::from(0.0001),
        }
    }
}
