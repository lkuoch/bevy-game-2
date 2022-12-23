use crate::prelude::*;

#[derive(
    Clone,
    Component,
    Copy,
    Derivative,
    Deserialize,
    FromReflect,
    Inspectable,
    PartialEq,
    Reflect,
    Serialize,
)]
#[derivative(Debug, Default)]
pub struct Player {
    #[derivative(Default(value = "10.0"))]
    pub move_speed: f32,

    pub variant: PlayerVariant,
}

#[derive(
    Clone,
    Component,
    Copy,
    Debug,
    Default,
    Deserialize,
    Eq,
    FromReflect,
    Inspectable,
    Hash,
    PartialEq,
    Reflect,
    Serialize,
)]
pub enum PlayerVariant {
    #[default]
    MaskDude,
    NinjaFrog,
    PinkMan,
    VirtualGuy,
}

#[derive(
    Clone,
    Component,
    Copy,
    Debug,
    Default,
    Deserialize,
    Eq,
    FromReflect,
    Inspectable,
    Hash,
    PartialEq,
    Reflect,
    Serialize,
)]
pub enum PlayerStates {
    DoubleJump,
    Fall,
    Jump,
    Hit,
    WallJump,
    #[default]
    Idle,
    Run,
}

pub mod player_states {
    use super::*;

    #[derive(Clone, Component, Debug, Deserialize, Reflect, Serialize)]
    #[component(storage = "SparseSet")]
    pub struct Idle;

    #[derive(Clone, Component, Debug, Deserialize, Reflect, Serialize)]
    #[component(storage = "SparseSet")]
    pub struct DoubleJump;

    #[derive(Clone, Component, Debug, Deserialize, Reflect, Serialize)]
    #[component(storage = "SparseSet")]
    pub struct Fall;

    #[derive(Clone, Component, Debug, Deserialize, Reflect, Serialize)]
    #[component(storage = "SparseSet")]
    pub struct Hit;

    #[derive(Clone, Component, Debug, Deserialize, Reflect, Serialize)]
    #[component(storage = "SparseSet")]
    pub struct Jump;

    #[derive(Clone, Component, Debug, Deserialize, Reflect, Serialize)]
    #[component(storage = "SparseSet")]
    pub struct WallJump;

    #[derive(Clone, Component, Debug, Deserialize, Reflect, Serialize)]
    #[component(storage = "SparseSet")]
    pub struct Run;

    #[derive(Clone, Copy, FromReflect, Reflect)]
    pub struct RunTrigger;

    #[derive(Clone, Copy, FromReflect, Reflect)]
    pub struct IdleTrigger;

    #[derive(Clone, Copy, FromReflect, Reflect)]
    pub struct FallTrigger;

    impl Player {
        pub fn new_state_machine(&self) -> StateMachine {
            StateMachine::new(player_states::Idle)
                .trans::<player_states::Idle>(RunTrigger, player_states::Run)
                .trans::<player_states::Idle>(FallTrigger, player_states::Fall)
                .trans::<player_states::Fall>(IdleTrigger, player_states::Idle)
                .trans::<player_states::Fall>(RunTrigger, player_states::Run)
                .trans::<player_states::Run>(NotTrigger(RunTrigger), player_states::Idle)
                .insert_on_enter::<player_states::Idle>(PlayerStates::Idle)
                .remove_on_exit::<player_states::Idle, PlayerStates>()
                .insert_on_enter::<player_states::Run>(PlayerStates::Run)
                .remove_on_exit::<player_states::Run, PlayerStates>()
                .insert_on_enter::<player_states::Fall>(PlayerStates::Fall)
                .remove_on_exit::<player_states::Fall, PlayerStates>()
        }
    }

    pub mod triggers {
        use super::*;

        impl Trigger for RunTrigger {
            type Param<'w, 's> = (
                Query<'w, 's, &'static KinematicCharacterControllerOutput>,
                Res<'w, Time>,
            );

            fn trigger(&self, _: Entity, (query, _time): &Self::Param<'_, '_>) -> bool {
                for controller in query.iter() {
                    if controller.effective_translation.x.abs() > 0.0 {
                        return true;
                    }
                }

                return false;
            }
        }

        impl Trigger for FallTrigger {
            type Param<'w, 's> = (
                Query<'w, 's, &'static KinematicCharacterControllerOutput>,
                Res<'w, Time>,
            );

            fn trigger(&self, _: Entity, (query, _time): &Self::Param<'_, '_>) -> bool {
                for controller in query.iter() {
                    if controller.effective_translation.y < 0.0 {
                        return true;
                    }
                }

                return false;
            }
        }

        impl Trigger for IdleTrigger {
            type Param<'w, 's> = (
                Query<'w, 's, &'static KinematicCharacterControllerOutput>,
                Res<'w, Time>,
            );

            fn trigger(&self, _: Entity, (query, _time): &Self::Param<'_, '_>) -> bool {
                for controller in query.iter() {
                    if controller.effective_translation == Vec2::ZERO {
                        return true;
                    }
                }

                return false;
            }
        }
    }
}
