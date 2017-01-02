// A simple test plugin to demonstrate how the hook system 
// can help extend the core application with custom code 
// defined in a plugin such as this

extern crate iron;
extern crate staticfile;
extern crate mount;

extern crate app_registry;
extern crate app_trait;

use std::path::Path;
use staticfile::Static;
use mount::Mount;

use app_trait::Plugin;
use app_trait::Hook;

use app_registry::hook_registry::{HOOK_REGISTRY};

pub struct MyTestPlugin {}

impl Plugin for MyTestPlugin {
    fn name(&self) -> &str {
        "MyTestPlugin v0.1 by soren"
    }

    fn init(&self) {
		println!("{}", "(my_test_plugin) fn: init()");
    	
    	HOOK_REGISTRY.lock().unwrap().action_mount_static_file_path.push(Box::new(ActionMountStaticFilePath::new(10)));
    	HOOK_REGISTRY.lock().unwrap().filter_the_content.push(Box::new(FilterTheContent::new(0)));
	}
}

pub struct ActionMountStaticFilePath {
	priority: i32
}

impl ActionMountStaticFilePath {
    fn new(p: i32) -> ActionMountStaticFilePath {
        ActionMountStaticFilePath {priority: p}
    }
}

impl Hook for ActionMountStaticFilePath {
    fn action_mount_static_file_path(&self, mount: &mut Mount){
    	println!("{}", "(my_test_plugin) fn: mount_action_mount_static_file_path()");
		mount.mount("/my_test_plugin/public", Static::new(Path::new("../my_test_plugin/public/")));
    }
    
    fn priority(&self) -> i32 {
        self.priority
    }
}

pub struct FilterTheContent {
    priority: i32
}

impl FilterTheContent {
    fn new(p: i32) -> FilterTheContent {
        FilterTheContent {priority: p}
    }
}

impl Hook for FilterTheContent {
    fn filter_the_content(&self, content: String) -> String {
        let owned_string: String = content.to_owned();
        let borrowed_string: &str = "world";

        //owned_string.push_str(borrowed_string);
        //owned_string

        let temp = "test".to_string();
        let new_owned_string = temp + &owned_string + borrowed_string;
        new_owned_string
    }
    
    fn priority(&self) -> i32 {
        self.priority
    }
}