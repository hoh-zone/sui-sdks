pub trait TransactionPlugin: Send + Sync {
    fn name(&self) -> String;

    fn before_build(&mut self, tx: &mut serde_json::Value) -> Result<(), String>;

    fn after_build(&mut self, tx: &serde_json::Value) -> Result<(), String>;
}

#[derive(Debug, Clone)]
pub struct NamedPackagesPlugin {
    pub packages: Vec<(String, String)>,
}

impl NamedPackagesPlugin {
    pub fn new() -> Self {
        Self {
            packages: Vec::new(),
        }
    }

    pub fn add_package(&mut self, name: String, address: String) {
        self.packages.push((name, address));
    }
}

impl Default for NamedPackagesPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl TransactionPlugin for NamedPackagesPlugin {
    fn name(&self) -> String {
        "NamedPackages".to_string()
    }

    fn before_build(&mut self, _tx: &mut serde_json::Value) -> Result<(), String> {
        Ok(())
    }

    fn after_build(&mut self, _tx: &serde_json::Value) -> Result<(), String> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_named_packages_plugin() {
        let mut plugin = NamedPackagesPlugin::new();
        plugin.add_package("sui".to_string(), "0x2".to_string());

        assert_eq!(plugin.packages.len(), 1);
        assert_eq!(plugin.packages[0].0, "sui");
        assert_eq!(plugin.packages[0].1, "0x2");
    }

    #[test]
    fn test_plugin_lifecycle() {
        let mut plugin = NamedPackagesPlugin::new();

        let mut tx = serde_json::json!({"test": "data"});

        assert!(plugin.before_build(&mut tx).is_ok());
        assert!(plugin.after_build(&tx).is_ok());
    }

    #[test]
    fn test_plugin_name() {
        let plugin = NamedPackagesPlugin::new();
        assert_eq!(plugin.name(), "NamedPackages");
    }
}
