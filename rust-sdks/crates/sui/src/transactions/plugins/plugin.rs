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

    fn before_build(&mut self, tx: &mut serde_json::Value) -> Result<(), String> {
        replace_named_packages(tx, &self.packages);
        Ok(())
    }

    fn after_build(&mut self, tx: &serde_json::Value) -> Result<(), String> {
        if let Some(name) = find_unresolved_named_package(tx) {
            return Err(format!("unresolved named package: {name}"));
        }
        Ok(())
    }
}

fn replace_named_packages(node: &mut serde_json::Value, packages: &[(String, String)]) {
    match node {
        serde_json::Value::Object(map) => {
            for value in map.values_mut() {
                replace_named_packages(value, packages);
            }
        }
        serde_json::Value::Array(arr) => {
            for value in arr.iter_mut() {
                replace_named_packages(value, packages);
            }
        }
        serde_json::Value::String(s) => {
            for (name, address) in packages {
                let needle = format!("{name}::");
                if s.contains(&needle) {
                    *s = s.replace(&needle, &format!("{address}::"));
                }
            }
        }
        _ => {}
    }
}

fn find_unresolved_named_package(node: &serde_json::Value) -> Option<String> {
    match node {
        serde_json::Value::Object(map) => map.values().find_map(find_unresolved_named_package),
        serde_json::Value::Array(arr) => arr.iter().find_map(find_unresolved_named_package),
        serde_json::Value::String(s) => {
            if let Some((package, _)) = s.split_once("::") {
                if package.contains('/') {
                    return Some(package.to_string());
                }
            }
            None
        }
        _ => None,
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
        plugin.add_package("mysten/sui".to_string(), "0x2".to_string());

        let mut tx = serde_json::json!({
            "package": "mysten/sui::coin::Coin",
            "typeArguments": ["mysten/sui::coin::Coin<mysten/sui::sui::SUI>"]
        });

        assert!(plugin.before_build(&mut tx).is_ok());
        assert!(plugin.after_build(&tx).is_ok());
        assert_eq!(tx["package"], "0x2::coin::Coin");
        assert_eq!(tx["typeArguments"][0], "0x2::coin::Coin<0x2::sui::SUI>");
    }

    #[test]
    fn test_plugin_name() {
        let plugin = NamedPackagesPlugin::new();
        assert_eq!(plugin.name(), "NamedPackages");
    }
}
