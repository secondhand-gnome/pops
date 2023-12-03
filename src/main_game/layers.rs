use bevy_rapier2d::geometry::Group;

pub enum Layer {
    Kernel,
    Skillet,
}

pub trait CollisionGroupMethods {
    fn join_groups(&self) -> Group;
}

impl Layer {
    pub fn group(&self) -> Group {
        match self {
            Self::Kernel => Group::GROUP_1,
            Self::Skillet => Group::GROUP_2,
        }
    }
    pub fn z(&self) -> f32 {
        match self {
            Self::Kernel => 2.,
            Self::Skillet => 1.,
        }
    }
}

impl CollisionGroupMethods for Vec<Layer> {
    fn join_groups(&self) -> Group {
        self.iter().fold(Group::NONE, |acc, x| acc | x.group())
    }
}
