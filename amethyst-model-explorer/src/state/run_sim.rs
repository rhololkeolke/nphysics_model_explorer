use crate::resource;
use amethyst::{
    input::is_key_down, renderer::VirtualKeyCode, GameData, SimpleState, SimpleTrans, StateData,
    StateEvent, Trans,
};

pub struct RunSimState;

impl SimpleState for RunSimState {
    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::R) {
                // restart the simulation
                println!("Restart simulation");

                // TODO(dschwab): Should probably filter the types of
                // entities I'm deleting by which component they have.
                data.world.delete_all();

                *data.world.write_resource::<resource::ReloadModel>() =
                    resource::ReloadModel::Restart;

                return Trans::Pop;
            } else if is_key_down(&event, VirtualKeyCode::L) {
                println!("Reload model");

                // TODO(dschwab): Should probably filter the types of
                // entities I'm deleting by which component they have.
                data.world.delete_all();

                // reload the model and restart
                *data.world.write_resource::<resource::ReloadModel>() =
                    resource::ReloadModel::Reload;

                return Trans::Pop;
            }
        }

        Trans::None
    }
}
