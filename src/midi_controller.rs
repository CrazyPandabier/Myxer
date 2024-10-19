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
    pub fn new(name: &str) -> Self {
        Application {
            name: name.to_string(),
        }
    }

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

    pub fn add_application(&mut self, group_name: &str, app: Application) {
        let group = self
            .groups
            .iter_mut()
            .find(|group| group.0.name == group_name);

        if let Some(group) = group {
            group.1.push(app);
        }
    }
}
