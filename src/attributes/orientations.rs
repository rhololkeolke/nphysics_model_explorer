use super::real_vector_attribute::{parse_real_vector_attribute, ParseRealAttributeError};
use failure::Fail;
use nalgebra as na;
use slog;
use slog::warn;

#[derive(Clone, PartialEq, Debug, Fail)]
pub enum ParseOrientationError {
    #[fail(display = "Multiple orientations specified")]
    MultipleOrientationsSpecified,
    #[fail(display = "{}", 0)]
    ParseRealAttributeError(#[fail(cause)] ParseRealAttributeError),
}

impl From<ParseRealAttributeError> for ParseOrientationError {
    fn from(error: ParseRealAttributeError) -> ParseOrientationError {
        ParseOrientationError::ParseRealAttributeError(error)
    }
}

/// Parses a tag's orientation into a unit quaternion.
///
/// Geoms and other objects in the MJCF XML have multiple ways of
/// specifying orientation. This method supports all of them, while
/// checking that only one of them is used on the tag.
pub fn parse_orientation_attribute<N: na::RealField>(
    logger: &slog::Logger,
    node: &roxmltree::Node,
    allow_fromto: bool,
) -> Result<na::UnitQuaternion<N>, ParseOrientationError>
where
    N: std::str::FromStr,
{
    let mut output: Option<na::UnitQuaternion<N>> = None;

    let quat = node.attribute("quat");
    if quat.is_some() && output.is_some() {
        return Err(ParseOrientationError::MultipleOrientationsSpecified);
    } else if quat.is_some() {
        // mujoco uses [w, x, y, z] but nalgebra uses [x, y, z, w]
        let mujoco_quat: na::Vector4<N> = parse_real_vector_attribute(quat.unwrap())?;

        let quat_vec = na::Vector3::<N>::from(mujoco_quat.fixed_rows::<na::U3>(1));
        let nphysics_quat = na::Quaternion::<N>::from_parts(*mujoco_quat.get(0).unwrap(), quat_vec);
        output = Some(na::UnitQuaternion::from_quaternion(nphysics_quat));
    }

    let axisangle = node.attribute("axisangle");
    if axisangle.is_some() && output.is_some() {
        return Err(ParseOrientationError::MultipleOrientationsSpecified);
    } else if axisangle.is_some() {
        let axisangle_values: na::Vector4<N> = parse_real_vector_attribute(axisangle.unwrap())?;
        let axis = na::Vector3::<N>::from(axisangle_values.fixed_rows::<na::U3>(0));
        let axis = na::Unit::new_normalize(axis);
        output = Some(na::UnitQuaternion::<N>::from_axis_angle(
            &axis,
            *axisangle_values.get(3).unwrap(),
        ));
    }

    let euler = node.attribute("euler");
    if euler.is_some() && output.is_some() {
        return Err(ParseOrientationError::MultipleOrientationsSpecified);
    } else if euler.is_some() {
        let euler_values: na::Vector3<N> = parse_real_vector_attribute(euler.unwrap())?;
        let roll = *euler_values.get(0).unwrap();
        let pitch = *euler_values.get(1).unwrap();
        let yaw = *euler_values.get(2).unwrap();

        // TODO(dschwab): handle the eulerseq compiler option
        output = Some(na::UnitQuaternion::<N>::from_euler_angles(roll, pitch, yaw));
    }

    let xyaxes = node.attribute("xyaxes");
    if xyaxes.is_some() && output.is_some() {
        return Err(ParseOrientationError::MultipleOrientationsSpecified);
    } else if xyaxes.is_some() {
        let xyaxes_values: na::Vector6<N> = parse_real_vector_attribute(xyaxes.unwrap())?;
        let x_axis = na::Vector3::<N>::from(xyaxes_values.fixed_rows::<na::U3>(0));
        let y_axis = na::Vector3::<N>::from(xyaxes_values.fixed_rows::<na::U3>(3));

        let z_axis: na::Vector3<N> = x_axis.cross(&y_axis);

        let rot_mat = na::Matrix3::<N>::from_columns(&[x_axis, y_axis, z_axis]);

        output = Some(na::UnitQuaternion::<N>::from_matrix_eps(
            &rot_mat,
            N::default_epsilon(),
            100,
            na::UnitQuaternion::identity(),
        ));
    }

    let zaxis = node.attribute("zaxis");
    if zaxis.is_some() && output.is_some() {
        return Err(ParseOrientationError::MultipleOrientationsSpecified);
    } else if zaxis.is_some() {
        let default_axis = na::Vector3::<N>::y();
        let zaxis: na::Vector3<N> = parse_real_vector_attribute(zaxis.unwrap())?;

        output = na::UnitQuaternion::<N>::rotation_between(&default_axis, &zaxis);
    }

    if allow_fromto {
        let fromto = node.attribute("fromto");
        if fromto.is_some() && output.is_some() {
            return Err(ParseOrientationError::MultipleOrientationsSpecified);
        } else if fromto.is_some() {
            let fromto_values: na::Vector6<N> = parse_real_vector_attribute(fromto.unwrap())?;
            let p0 = na::Vector3::<N>::from(fromto_values.fixed_rows::<na::U3>(0));
            let p1 = na::Vector3::<N>::from(fromto_values.fixed_rows::<na::U3>(3));

            let default_axis = na::Vector3::<N>::y();
            let zaxis: na::Vector3<N> = p1 - p0;
            let zaxis = zaxis.normalize();

            output = na::UnitQuaternion::<N>::rotation_between(&default_axis, &zaxis);
        }
    } else if node.has_attribute("fromto") {
        warn!(logger, "allow_fromto is false, but fromto attribute is present."; "node" => ?node);
    }

    match output {
        Some(output) => Ok(output),
        None => Ok(na::UnitQuaternion::<N>::identity()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::log;
    use nalgebra as na;
    use proptest::prelude::*;
    use proptest::prop_compose;
    use roxmltree;

    impl From<ParseOrientationError> for TestCaseError {
        fn from(error: ParseOrientationError) -> TestCaseError {
            TestCaseError::fail(format!("{}", error))
        }
    }

    prop_compose! {
        fn multiple_orientations_strategy()
            (enabled_orientations in proptest::collection::vec(proptest::bool::ANY, 6),
             quat_values in proptest::collection::vec(proptest::num::f32::NORMAL, 4),
             // axis angle values
             axis_values in proptest::collection::vec(proptest::num::f32::NORMAL, 3),
             angle in proptest::num::f32::NORMAL,
             // euler angles
             roll in 0f32..std::f32::consts::FRAC_2_PI,
             pitch in 0f32..std::f32::consts::FRAC_2_PI,
             yaw in 0f32..std::f32::consts::FRAC_2_PI,
             // xyaxes
             x_values in proptest::collection::vec(-1.0f32..1.0, 3),
             y_values in proptest::collection::vec(-1.0f32..1.0, 3),
             // zaxis
             z_values in proptest::collection::vec(-1.0f32..1.0, 3),
             // fromto
             fromto in proptest::collection::vec(-10.0f32..10.00, 6)) -> Vec<String> {
                let mut attributes = Vec::<String>::new();

                if enabled_orientations[0] {
                    attributes.push(format!("quat=\"{} {} {} {}\"", quat_values[0], quat_values[1], quat_values[2], quat_values[3]));
                }

                if enabled_orientations[1] {
                    attributes.push(format!("axisangle=\"{} {} {} {}\"", axis_values[0], axis_values[1], axis_values[2], angle));
                }

                if enabled_orientations[2] {
                    attributes.push(format!("euler=\"{} {} {}\"", roll, pitch, yaw));
                }

                if enabled_orientations[3] {
                    attributes.push(format!("xyaxes=\"{} {} {} {} {} {}\"", x_values[0], x_values[1], x_values[2], y_values[0], y_values[1], y_values[2]));
                }

                if enabled_orientations[4] {
                    attributes.push(format!("zaxis=\"{} {} {}\"", z_values[0], z_values[1], z_values[2]));
                }

                if enabled_orientations[5] {
                    attributes.push(format!("fromto=\"{} {} {} {} {} {}\"", fromto[0], fromto[1], fromto[2], fromto[3], fromto[4], fromto[5]));
                }

                attributes
            }
    }

    #[test]
    fn parse_no_orientation() {
        let expected_quat = na::UnitQuaternion::<f32>::identity();

        let xml = "<geom/>";
        let doc = roxmltree::Document::parse(&xml).unwrap();
        let node = doc.root_element();

        let quat = parse_orientation_attribute(&**log::LOG.read().unwrap(), &node, true).unwrap();
        assert_eq!(quat, expected_quat);
    }

    proptest! {
        #[test]
        fn parse_quat(quat_values in proptest::collection::vec(proptest::num::f32::NORMAL, 4)) {
            let w = quat_values[0];
            let x = quat_values[1];
            let y = quat_values[2];
            let z = quat_values[3];

            let expected_quat = na::Quaternion::from_parts(w, na::Vector3::new(x, y, z));
            let expected_quat = na::UnitQuaternion::from_quaternion(expected_quat);

            let xml = format!("<geom quat=\"{} {} {} {}\" />", w, x, y, z);
            let doc = roxmltree::Document::parse(&xml)?;
            let node = doc.root_element();

            let quat = parse_orientation_attribute(&**log::LOG.read().unwrap(), &node, true)?;
            prop_assert_eq!(quat, expected_quat);
        }

        #[test]
        fn parse_axisangle(axis_values in proptest::collection::vec(proptest::num::f32::NORMAL, 3), angle in proptest::num::f32::NORMAL) {
            let axis = na::Unit::new_normalize(na::Vector3::new(axis_values[0], axis_values[1], axis_values[2]));
            let expected_quat = na::UnitQuaternion::from_axis_angle(&axis, angle);

            let xml = format!("<geom axisangle=\"{} {} {} {}\" />", axis_values[0], axis_values[1], axis_values[2], angle);
            let doc = roxmltree::Document::parse(&xml)?;
            let node = doc.root_element();

            let quat = parse_orientation_attribute(&**log::LOG.read().unwrap(), &node, true)?;
            prop_assert_eq!(quat, expected_quat);
        }

        #[test]
        fn parse_euler(roll in 0f32..std::f32::consts::FRAC_2_PI, pitch in 0f32..std::f32::consts::FRAC_2_PI, yaw in 0f32..std::f32::consts::FRAC_2_PI) {
            let expected_quat = na::UnitQuaternion::from_euler_angles(roll, pitch, yaw);

            let xml = format!("<geom euler=\"{} {} {}\"/>", roll, pitch, yaw);
            let doc = roxmltree::Document::parse(&xml)?;
            let node = doc.root_element();

            let quat = parse_orientation_attribute(&**log::LOG.read().unwrap(), &node, true)?;
            prop_assert_eq!(quat, expected_quat);
        }

        #[test]
        fn parse_xyaxes(x_values in proptest::collection::vec(-1.0f32..1.0, 3),
                        y_values in proptest::collection::vec(-1.0f32..1.0, 3)) {
            let x_axis = na::Vector3::new(x_values[0], x_values[1], x_values[2]);
            let y_axis = na::Vector3::new(y_values[0], y_values[1], y_values[2]);
            let z_axis = x_axis.cross(&y_axis);
            let rot_mat = na::Matrix3::from_columns(&[x_axis, y_axis, z_axis]);
            let expected_quat = na::UnitQuaternion::from_matrix_eps(&rot_mat,
                                                                    std::f32::EPSILON,
                                                                    100,
                                                                    na::UnitQuaternion::identity());

            let xml = format!("<geom xyaxes=\"{} {} {} {} {} {}\"/>",
                              x_values[0],
                              x_values[1],
                              x_values[2],
                              y_values[0],
                              y_values[1],
                              y_values[2]);
            let doc = roxmltree::Document::parse(&xml)?;
            let node = doc.root_element();

            let quat = parse_orientation_attribute(&**log::LOG.read().unwrap(), &node, true)?;
            prop_assert_eq!(quat, expected_quat);
        }

        #[test]
        fn parse_zaxis(z_values in proptest::collection::vec(-1.0f32..1.0, 3)) {
            let z_axis = na::Vector3::new(z_values[0], z_values[1], z_values[2]);
            let default_axis = na::Vector3::y();
            let expected_quat = na::UnitQuaternion::rotation_between(&default_axis, &z_axis).unwrap_or(na::UnitQuaternion::<f32>::identity());

            let xml = format!("<geom zaxis=\"{} {} {}\"/>", z_values[0], z_values[1], z_values[2]);
            let doc = roxmltree::Document::parse(&xml)?;
            let node = doc.root_element();

            let quat = parse_orientation_attribute(&**log::LOG.read().unwrap(), &node, true)?;
            prop_assert_eq!(quat, expected_quat);
        }

        #[test]
        fn parse_fromto_when_enabled(fromto in proptest::collection::vec(-10.0f32..10.00, 6)) {
            let p0 = na::Point3::new(fromto[0], fromto[1], fromto[2]);
            let p1 = na::Point3::new(fromto[3], fromto[4], fromto[5]);
            let z_axis = (p1 - p0).normalize();
            let default_axis = na::Vector3::y();
            let expected_quat = na::UnitQuaternion::rotation_between(&default_axis, &z_axis).unwrap_or(na::UnitQuaternion::<f32>::identity());

            let xml = format!("<geom fromto=\"{} {} {} {} {} {}\"/>", fromto[0], fromto[1], fromto[2], fromto[3], fromto[4], fromto[5]);
            let doc = roxmltree::Document::parse(&xml)?;
            let node = doc.root_element();

            let quat = parse_orientation_attribute(&**log::LOG.read().unwrap(), &node, true)?;
            prop_assert_eq!(quat, expected_quat);
        }

        #[test]
        fn parse_fromto_ignored_when_disabled(fromto in proptest::collection::vec(-10.0f32..10.00, 6)) {
            let p0 = na::Point3::new(fromto[0], fromto[1], fromto[2]);
            let p1 = na::Point3::new(fromto[3], fromto[4], fromto[5]);
            let z_axis = p1 - p0;
            prop_assume!(z_axis.magnitude() > 0.5);
            let default_axis = na::Vector3::z();
            let fromto_quat = na::UnitQuaternion::rotation_between(&default_axis, &z_axis).unwrap();
            prop_assume!(fromto_quat != na::UnitQuaternion::identity());

            let xml = format!("<geom fromto=\"{} {} {} {} {} {}\"/>", fromto[0], fromto[1], fromto[2], fromto[3], fromto[4], fromto[5]);
            let doc = roxmltree::Document::parse(&xml)?;
            let node = doc.root_element();

            let quat = parse_orientation_attribute(&**log::LOG.read().unwrap(), &node, false)?;
            prop_assert_eq!(quat, na::UnitQuaternion::<f32>::identity());
        }

        #[test]
        fn parse_multiple_orientations_error(orientation_attributes in multiple_orientations_strategy()) {
            prop_assume!(orientation_attributes.len() >= 2);

            let all_attributes = orientation_attributes.join(" ");
            let xml = format!("<geom {}/>", all_attributes);
            let doc = roxmltree::Document::parse(&xml)?;
            let node = doc.root_element();

            if let Err(error) = parse_orientation_attribute::<f32>(&**log::LOG.read().unwrap(), &node, true) {
                match error {
                    ParseOrientationError::MultipleOrientationsSpecified => {},
                    _ => return Err(TestCaseError::fail(format!("Unexpected parsing error: {}", error))),
                }
            } else {
                return Err(TestCaseError::fail("Parse successfully despite there being multiple orientations defined"));
            }
        }
    }
}
