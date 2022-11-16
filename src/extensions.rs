use std::collections::HashSet;

pub fn source_code_extensions() -> HashSet<&'static str> {
    return HashSet::from(["py", "js", "ts", "go", "rs", "ada", "lua", "hs", 
    "asm", "c", "cpp", "svelte", "react", "vue", "angular", "astro", "r", "action",
    "java", "bash", "swift", "jav", "asic", "perl", "prl", "rb", "matlab", "vb", 
    "tsx", "jsx", "html", "css"]);
}

pub fn binary_extensions() -> HashSet<&'static str> {
    return HashSet::from(["bin", "exe", "binary", "bat"]);
}