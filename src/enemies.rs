use crate::prelude::*;

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Inspectable, Hash, PartialEq, Serialize)]
pub enum EnemyType {
    #[default]
    AngryPig,
    Bat,
    Bee,
    BlueBird,
    Bunny,
    Chameleon,
    Chicken,
    Duck,
    FatBird,
    Ghost,
    Mushroom,
    Plant,
    Radish,
    Rhino,
    Rock1,
    Rock2,
    Rock3,
    Skull,
    Slime,
    Snail,
    Trunk,
    Turtle,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Inspectable, Hash, PartialEq)]
pub enum EnemyState {
    AngryPig(AngryPigState),
    Bat(BatState),
    Bee(BeeState),
    BlueBird(BlueBirdState),
    Bunny(BunnyState),
    Chameleon(ChameleonState),
    Chicken(ChickenState),
    Duck(DuckState),
    FatBird(FatBirdState),
    Ghost(GhostState),
    Mushroom(MushroomState),
    Plant(PlantState),
    Radish(RadishState),
    Rhino(RhinoState),
    Rock1(Rock1State),
    Rock2(Rock2State),
    Rock3(Rock3State),
    Skull(SkullState),
    Slime(SlimeState),
    Snail(SnailState),
    Trunk(TrunkState),
    Turtle(TurtleState),
}

impl EnemyState {
    pub fn get_enemy_type(&self) -> EnemyType {
        match *self {
            EnemyState::AngryPig(_) => EnemyType::AngryPig,
            EnemyState::Bat(_) => EnemyType::Bat,
            EnemyState::Bee(_) => EnemyType::Bee,
            EnemyState::BlueBird(_) => EnemyType::BlueBird,
            EnemyState::Bunny(_) => EnemyType::Bunny,
            EnemyState::Chameleon(_) => EnemyType::Chameleon,
            EnemyState::Chicken(_) => EnemyType::Chicken,
            EnemyState::Duck(_) => EnemyType::Duck,
            EnemyState::FatBird(_) => EnemyType::FatBird,
            EnemyState::Ghost(_) => EnemyType::Ghost,
            EnemyState::Mushroom(_) => EnemyType::Mushroom,
            EnemyState::Plant(_) => EnemyType::Plant,
            EnemyState::Radish(_) => EnemyType::Radish,
            EnemyState::Rhino(_) => EnemyType::Rhino,
            EnemyState::Rock1(_) => EnemyType::Rock1,
            EnemyState::Rock2(_) => EnemyType::Rock2,
            EnemyState::Rock3(_) => EnemyType::Rock3,
            EnemyState::Skull(_) => EnemyType::Skull,
            EnemyState::Slime(_) => EnemyType::Slime,
            EnemyState::Snail(_) => EnemyType::Snail,
            EnemyState::Trunk(_) => EnemyType::Trunk,
            EnemyState::Turtle(_) => EnemyType::Turtle,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Inspectable, Hash, PartialEq)]
pub enum AngryPigState {
    Hit1,
    Hit2,
    #[default]
    Idle,
    Run,
    Walk,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Inspectable, Hash, PartialEq)]
pub enum BatState {
    CeilingIn,
    CeilingOut,
    Flying,
    Hit,
    #[default]
    Idle,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Inspectable, Hash, PartialEq)]
pub enum BeeState {
    Attack,
    Hit,
    #[default]
    Idle,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Inspectable, Hash, PartialEq)]
pub enum BlueBirdState {
    #[default]
    Flying,
    Hit,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Inspectable, Hash, PartialEq)]
pub enum BunnyState {
    Fall,
    Hit,
    #[default]
    Idle,
    Jump,
    Run,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Inspectable, Hash, PartialEq)]
pub enum ChameleonState {
    Attack,
    Hit,
    #[default]
    Idle,
    Run,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Inspectable, Hash, PartialEq)]
pub enum ChickenState {
    Hit,
    #[default]
    Idle,
    Run,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Inspectable, Hash, PartialEq)]
pub enum DuckState {
    Fall,
    Hit,
    #[default]
    Idle,
    Jump,
    JumpAnticipation,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Inspectable, Hash, PartialEq)]
pub enum FatBirdState {
    Fall,
    Ground,
    Hit,
    #[default]
    Idle,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Inspectable, Hash, PartialEq)]
pub enum GhostState {
    Appear,
    Disappear,
    Hit,
    #[default]
    Idle,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Inspectable, Hash, PartialEq)]
pub enum MushroomState {
    Hit,
    #[default]
    Idle,
    Run,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Inspectable, Hash, PartialEq)]
pub enum PlantState {
    Attack,
    Hit,
    #[default]
    Idle,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Inspectable, Hash, PartialEq)]
pub enum RadishState {
    Hit,
    #[default]
    Idle1,
    Idle2,
    Run,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Inspectable, Hash, PartialEq)]
pub enum RhinoState {
    Hit,
    HitWall,
    #[default]
    Idle,
    Run,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Inspectable, Hash, PartialEq)]
pub enum Rock1State {
    Hit,
    #[default]
    Idle,
    Run,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Inspectable, Hash, PartialEq)]
pub enum Rock2State {
    Hit,
    #[default]
    Idle,
    Run,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Inspectable, Hash, PartialEq)]
pub enum Rock3State {
    Hit,
    #[default]
    Idle,
    Run,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Inspectable, Hash, PartialEq)]
pub enum SkullState {
    Hit,
    HitWall1,
    HitWall2,
    #[default]
    Idle1,
    Idle2,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Inspectable, Hash, PartialEq)]
pub enum SlimeState {
    Hit,
    #[default]
    IdleRun,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Inspectable, Hash, PartialEq)]
pub enum SnailState {
    Hit,
    #[default]
    Idle,
    ShellIdle,
    ShellTopHit,
    ShellWallHit,
    NoShell,
    Walk,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Inspectable, Hash, PartialEq)]
pub enum TrunkState {
    Attack,
    Hit,
    #[default]
    Idle,
    Run,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Inspectable, Hash, PartialEq)]
pub enum TurtleState {
    Hit,
    #[default]
    Idle1,
    Idle2,
    SpikesIn,
    SpikesOut,
}
