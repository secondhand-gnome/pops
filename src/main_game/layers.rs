use bevy_rapier2d::geometry::Group;

pub enum Layer {
    RawKernel,
    PoppedKernel,
    Skillet,
}

pub trait CollisionGroupMethods {
    fn group(&self) -> Group;
}

impl Layer {
    fn group(&self) -> Group {
        match self {
            Self::RawKernel => Group::GROUP_1,
            Self::PoppedKernel => Group::GROUP_2,
            Self::Skillet => Group::GROUP_3,
        }
    }
    pub fn z(&self) -> f32 {
        match self {
            Self::RawKernel => 3.,
            Self::PoppedKernel => 2.,
            Self::Skillet => 1.,
        }
    }
}

impl CollisionGroupMethods for Vec<Layer> {
    fn group(&self) -> Group {
        self.iter().fold(Group::NONE, |acc, x| acc | x.group())
    }
}
