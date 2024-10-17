use std::collections::HashMap;
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Group {
    name: String,
    volume: Option<String>,
    mute: Option<String>,
}

impl Group {
    pub fn new(name: &str) -> Self {
        Group {
            name: name.to_string(),
            volume: None,
            mute: None,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}
#[derive(Clone)]
pub struct Application {
    name: String,
}

impl Application {
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

#[derive(Clone)]
pub struct ControllerProfile {
    name: String,
    groups: HashMap<Group, Vec<Application>>,
}

impl ControllerProfile {
    pub fn new(name: &str, groups: Vec<Group>) -> Self {
        let groups: HashMap<Group, Vec<Application>> = groups
            .iter()
            .map(|group| (group.clone(), Vec::new()))
            .collect();

        ControllerProfile {
            name: name.to_string(),
            groups,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_groups(&self) -> &HashMap<Group, Vec<Application>> {
        &self.groups
    }
}
