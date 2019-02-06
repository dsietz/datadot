use dynamic_reload::{DynamicReload, Lib, Symbol, Search, PlatformName, UpdateState};
use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::Duration;
use std::thread;

pub struct Plugins {
    pub plugins: Vec<Arc<Lib>>,
}

impl Plugins {
    fn add_plugin(&mut self, lib: &Arc<Lib>) {
        self.plugins.push(lib.clone());
        //self.plugins.insert(name, lib.clone());
    }

    fn clear(&mut self){
        self.plugins.clear();
    }

    fn get_plugins(&mut self) -> Vec<Arc<Lib>>{
        self.plugins.clone()
    }

    fn get_plugin_count(&self) -> usize {
        self.plugins.len()
    }

    fn unload_plugin(&mut self, lib: &Arc<Lib>) {
        
        for i in (0..self.plugins.len()).rev() {
            if &self.plugins[i] == lib {
                self.plugins.swap_remove(i);
            }
        }
    }

    fn reload_plugin(&mut self, lib: &Arc<Lib>) {
        Self::add_plugin(self, lib);
    }

    // called when a lib needs to be reloaded.
    pub fn reload_callback(&mut self, state: UpdateState, lib: Option<&Arc<Lib>>) {
        match state {
            UpdateState::Before => Self::unload_plugin(self, lib.unwrap()),
            UpdateState::After => Self::reload_plugin(self, lib.unwrap()),
            UpdateState::ReloadFailed(_) => println!("Failed to reload"),
        }
    }
}

pub struct PluginManager<'a> {
    reload_handler: DynamicReload<'a>,
    plugins: Plugins,
}

impl <'a> Drop for PluginManager<'a> {
    fn drop(&mut self){
        //self.plugins.clear();
        self.unload();
    }
}

impl<'a> PluginManager<'a> {
    pub fn new(lib_dir_pathes: Vec<&'a str>, cache_dir_path: &'a str) -> PluginManager<'a>{
        PluginManager {
            reload_handler: DynamicReload::new(Some(lib_dir_pathes),
                                               None,
                                               Search::Backwards),
            plugins: Plugins { plugins: Vec::new() },
        }
    }

    pub fn add_plugin(&mut self, name: &str) -> std::result::Result<i32, &str>{
        match &self.reload_handler.add_library(name, PlatformName::Yes) {
            Ok(lib) => {
                    self.plugins.add_plugin(&lib);
                    Ok(1)
                },
            Err(e) => {
                println!("Unable to load dynamic lib {}, err {:?}", name, e);
                Err("Warning: Unable to load dynamic lib!")
            }
        }
    }

    pub fn get_plugin_count(&self) -> usize {
        self.plugins.get_plugin_count()
    } 

    pub fn get_plugin(&mut self) -> Arc<Lib>{
        self.plugins.plugins[0].clone()
    }

    pub fn get_plugins(&mut self) -> Vec<Arc<Lib>>{
        self.plugins.plugins.clone()
    }

    pub fn refresh_plugins(&mut self){
        &self.reload_handler.update(Plugins::reload_callback, &mut self.plugins);
    }

    pub fn unload(&mut self) -> std::result::Result<i32, &'static str>{
        debug!("Unloading plugins");

        for plugin in self.plugins.get_plugins() {
            trace!("Firing on_plugin_unload for {:?}", plugin.original_path.as_ref());
            //plugin.on_plugin_unload();
        }

        self.plugins.clear();
        Ok(1)
    }
}

#[cfg(test)]
mod tests {
    use super::PluginManager;
    use dynamic_reload::{Lib,Symbol};

    #[ignore]
    #[test]
    fn test_01() {
        assert!(true)
    }
}
