use std::io::sink;

use glib::clone;
use gtk::prelude::*;

use crate::midi_controller::{Application, ControllerProfile};
use crate::pulse::Pulse;
use crate::shared::Shared;

/**
 * The Card MidiProfiles popup window.
 * Allows creating new and updating profiles for midi controllers
 */

pub struct MidiProfiles {
    pulse: Shared<Pulse>,

    /** Indicates if the popup should remain open. */
    live: Shared<bool>,
}

impl MidiProfiles {
    /**
     * Creates the Midi Profiles window, and its contents.
     */
    pub fn new(
        parent: &gtk::ApplicationWindow,
        pulse: &Shared<Pulse>,
        profiles: &Shared<Vec<ControllerProfile>>,
    ) -> Self {

        let sink_input_names: Vec<String> = pulse
            .borrow()
            .sink_inputs
            .iter()
            .map(|sink_input| sink_input.1.data.description.clone())
            .collect();

        let dialog = gtk::Dialog::with_buttons(
            Some("Midi Profiles"),
            Some(parent),
            gtk::DialogFlags::all(),
            &[],
        );
        dialog.set_border_width(0);

        let live = Shared::new(true);
        dialog.connect_response(|s, _| s.emit_close());
        let live_clone = live.clone();
        dialog.connect_close(move |_| {
            live_clone.replace(false);
        });

        let geom = gdk::Geometry {
            min_width: 450,
            min_height: 550,
            max_width: 450,
            max_height: 10000,
            base_width: -1,
            base_height: -1,
            width_inc: -1,
            height_inc: -1,
            min_aspect: 0.0,
            max_aspect: 0.0,
            win_gravity: gdk::Gravity::Center,
        };

        dialog.set_geometry_hints::<gtk::Dialog>(
            None,
            Some(&geom),
            gdk::WindowHints::MIN_SIZE | gdk::WindowHints::MAX_SIZE,
        );

        // Create a ComboBoxText (dropdown) for selecting a profile
        let profile_dropdown = gtk::ComboBoxText::new();
        for profile in profiles.borrow().iter() {
            profile_dropdown.append_text(profile.get_name());
        }
        profile_dropdown.set_active(Some(0)); // Set the first profile as selected by default

        // Create a ScrolledWindow for displaying the groups and applications
        let scroller = gtk::ScrolledWindow::new::<gtk::Adjustment, gtk::Adjustment>(None, None);
        scroller.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);
        let content_area = dialog.get_content_area();
        content_area.pack_start(&profile_dropdown, false, false, 5);
        content_area.pack_start(&scroller, true, true, 5);
        content_area.set_border_width(0);

        // Placeholder Box for showing groups and their applications
        let group_box = gtk::Box::new(gtk::Orientation::Vertical, 5);
        scroller.add(&group_box);

        // Connect the signal to listen for profile selection changes
        profile_dropdown.connect_changed(
            clone!(@strong profiles, @strong sink_input_names, @weak group_box => move |dropdown| {
                // Clear the current content in group_box
                group_box.foreach(|child| group_box.remove(child));

                // Get the selected profile
                if let Some(selected_profile) = dropdown.get_active_text() {
                    let profiles =  profiles.borrow();
                    let selected: Option<&ControllerProfile> = profiles
                    .iter()
                    .find(|&profile| profile.get_name() == selected_profile);
                    // Fetch groups and applications for the selected profile
                    // In this example, we’ll just simulate some data based on the profile
                    let groups = get_groups_for_profile(selected.unwrap());

                    // Dynamically add groups and applications to the UI
                    for (group_name, apps) in groups {
                        let group_label = gtk::Label::new(Some(&format!("Group: {}", group_name)));
                        group_box.pack_start(&group_label, false, false, 5);
                    
                        // Box for applications in the group
                        let app_box = gtk::Box::new(gtk::Orientation::Vertical, 5);
                        group_box.pack_start(&app_box, false, false, 5);
                    
                        // Add each app with a remove button
                        for app in apps {
                            let app_row = gtk::Box::new(gtk::Orientation::Horizontal, 5);
                    
                            let app_label = gtk::Label::new(Some(app));
                            let remove_button = gtk::Button::with_label("Remove");
                            
                            // Handle remove action
                            remove_button.connect_clicked(clone!(@weak app_box, @weak app_row => move |_| {
                                // Logic to remove the app from the group (your internal data structure)
                                // Remove from the UI
                                app_box.remove(&app_row);
                            }));
                    
                            app_row.pack_start(&app_label, true, true, 5);
                            app_row.pack_start(&remove_button, false, false, 5);
                            app_box.pack_start(&app_row, false, false, 5);
                        }
                    
                        // Dropdown for selecting an app to add
                        let add_box = gtk::Box::new(gtk::Orientation::Horizontal, 5);
                        let app_dropdown = gtk::ComboBoxText::new();
                    
                        for app in &sink_input_names {
                            app_dropdown.append_text(app);
                        }
                    
                        let add_button = gtk::Button::with_label("Add");
                    
                        // Handle add action
                        add_button.connect_clicked(clone!(@weak app_box, @strong app_dropdown => move |_| {
                            if let Some(selected_app) = app_dropdown.get_active_text() {
                                let app_row = gtk::Box::new(gtk::Orientation::Horizontal, 5);
                                let app_label = gtk::Label::new(Some(&selected_app));
                                let remove_button = gtk::Button::with_label("Remove");

                                remove_button.connect_clicked(clone!(@weak app_box, @weak app_row => move |_| {
                                    // Logic to remove the app from the group (your internal data structure)
                                    // Remove from the UI
                                    app_box.remove(&app_row);
                                }));
                    
                                // Add new app to internal structure and UI
                                app_row.pack_start(&app_label, true, true, 5);
                                app_row.pack_start(&remove_button, false, false, 5);
                                app_box.pack_start(&app_row, false, false, 5);
                                app_box.show_all();
                    
                                // Logic to link the app to the group in your internal data structure
                            }
                        }));
                    
                        add_box.pack_start(&app_dropdown, true, true, 5);
                        add_box.pack_start(&add_button, false, false, 5);
                        group_box.pack_start(&add_box, false, false, 5);
                    }
                    
                    group_box.show_all();
                }
            }),
        );

        dialog.show_all();

        Self {
            live,
            pulse: pulse.clone(),
        }
    }
}

// Example function to simulate fetching groups and applications based on profile
fn get_groups_for_profile(profile: &ControllerProfile) -> Vec<(&str, Vec<&str>)> {
    let mut groups = Vec::new();
    for group in profile.get_groups() {
        let group_name = group.0.get_name();
        let app_names: Vec<&str> = group.1.iter().map(|app| app.get_name()).collect();
        groups.push((group_name, app_names));
    }
    groups
}
