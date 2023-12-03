use bevy_rapier2d::geometry::Group;

pub enum CollisionGroup {
    Kernel,
}

pub trait CollisionGroupMethods {
    fn join_groups(&self) -> Group;
}

impl CollisionGroup {
    pub fn group(&self) -> Group {
        match self {
            Self::Kernel => Group::GROUP_1,
        }
    }
}

impl CollisionGroupMethods for Vec<CollisionGroup> {
    fn join_groups(&self) -> Group {
        self.iter().fold(Group::NONE, |acc, x| acc | x.group())
    }
}
