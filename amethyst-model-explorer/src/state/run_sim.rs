use amethyst::{
    input::is_key_down, renderer::VirtualKeyCode, GameData, SimpleState, SimpleTrans, StateData,
    StateEvent, Trans,
};

pub struct RunSimState;

impl SimpleState for RunSimState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        // TODO(dschwab): Is this necessary?
        // might just be an empty method
        // otherwise, maybe setup cameras and rendering stuff?
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::R) {
                // restart the simulation
                println!("Restart simulation");
                return Trans::Pop;
            } else if is_key_down(&event, VirtualKeyCode::L) {
                println!("Reload model");
                // reload the model and restart

                // TODO(dschwab): How do a pop two states? Maybe emit
                // a custom event? and then check the event in the
                // other states? Maybe I can implement two copies of
                // the State trait one for popping and only implement
                // on_resume and in the other case implement normal?
                // Maybe I can make a custom game data struct that has
                // a flag for what kind of pop this is
                return Trans::Pop;
            }
        }
        

        Trans::None
    }
}
