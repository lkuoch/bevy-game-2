use crate::prelude::*;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(StateMachinePlugin)
            .add_plugin(TriggerPlugin::<player_states::RunTrigger>::default())
            .add_plugin(TriggerPlugin::<player_states::FallTrigger>::default())
            .add_plugin(TriggerPlugin::<player_states::IdleTrigger>::default());
    }
}
