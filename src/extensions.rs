use std::collections::HashSet;

pub fn source_code_extensions() -> HashSet<&'static str> {
    return HashSet::from(["py", "js", "ts", "go", "rs", "ada", "lua", "hs", 
    "asm", "c", "cpp", "svelte", "react", "vue", "angular", "r", "action", "java", 
    "bash", "swift", "jav", "asic", "perl", "prl", "rb", "matlab"]);
}

pub fn storage_extensions() -> HashSet<&'static str> {
    return HashSet::from(["json", "xml", "toml"]);
}

pub fn binary_extensions() -> HashSet<&'static str> {
    return HashSet::from(["bin", "exe", "binary", "bat"]);
}