use crate::resource::{self, mouse_drag};
use amethyst::{
    ecs::prelude::{Read, System, Write},
    input::InputHandler,
    renderer::MouseButton,
    core::nalgebra as na,
};
use sm::{AsEnum, Transition};

pub struct MouseDrag {
    button: MouseButton,
}

impl MouseDrag {
    pub fn new(button: MouseButton) -> Self {
        Self { button }
    }
}

impl Default for MouseDrag {
    fn default() -> Self {
        Self::new(MouseButton::Left)
    }
}

impl<'s> System<'s> for MouseDrag {
    type SystemData = (
        Read<'s, InputHandler<(), ()>>,
        Write<'s, resource::mouse_drag::Variant>,
    );

    fn run(&mut self, (input_handler, mut mouse_drag): Self::SystemData) {
        if let Some((mouse_pos_x, mouse_pos_y)) = input_handler.mouse_position() {
            let mouse_pos = na::Point2::<f64>::new(mouse_pos_x, mouse_pos_y);

            if input_handler.mouse_button_is_down(self.button) {
                let event = mouse_drag::Clicked {
                    position: mouse_pos,
                };

                *mouse_drag = match (*mouse_drag).clone() {
                    mouse_drag::Variant::InitialNotDragging(s) => s.transition(event).as_enum(),
                    mouse_drag::Variant::NotDraggingByNotClicked(s) => {
                        s.transition(event).as_enum()
                    }
                    mouse_drag::Variant::StartDraggingByClicked(s) => s.transition(event).as_enum(),
                    mouse_drag::Variant::DraggingByClicked(s) => s.transition(event).as_enum(),
                    mouse_drag::Variant::FinishDraggingByNotClicked(s) => {
                        s.transition(event).as_enum()
                    }
                }
            } else {
                let event = mouse_drag::NotClicked {
                    position: mouse_pos,
                };

                *mouse_drag = match (*mouse_drag).clone() {
                    mouse_drag::Variant::InitialNotDragging(s) => s.transition(event).as_enum(),
                    mouse_drag::Variant::NotDraggingByNotClicked(s) => {
                        s.transition(event).as_enum()
                    }
                    mouse_drag::Variant::StartDraggingByClicked(s) => s.transition(event).as_enum(),
                    mouse_drag::Variant::DraggingByClicked(s) => s.transition(event).as_enum(),
                    mouse_drag::Variant::FinishDraggingByNotClicked(s) => {
                        s.transition(event).as_enum()
                    }
                }
            }
        }
    }
}
