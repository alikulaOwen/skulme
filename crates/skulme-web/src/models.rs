use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SupportedLanguage {
    Rust,
    Python,
    Java,
    TypeScript,
}

impl SupportedLanguage {
    pub fn display_name(&self) -> &'static str {
        match self {
            SupportedLanguage::Rust => "Rust",
            SupportedLanguage::Python => "Python",
            SupportedLanguage::Java => "Java",
            SupportedLanguage::TypeScript => "TypeScript",
        }
    }

    pub fn extension(&self) -> &'static str {
        match self {
            SupportedLanguage::Rust => "rs",
            SupportedLanguage::Python => "py",
            SupportedLanguage::Java => "java",
            SupportedLanguage::TypeScript => "ts",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExampleCase {
    pub input: &'static str,
    pub output: &'static str,
    pub explanation: Option<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProblemHint {
    pub title: &'static str,
    pub content: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestCase {
    pub name: &'static str,
    pub input: &'static str,
    pub expected: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestResult {
    pub passed: bool,
    pub name: String,
    pub duration_ms: u64,
    pub input: String,
    pub expected: String,
    pub actual: String,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Problem {
    pub slug: &'static str,
    pub title: &'static str,
    pub category: &'static str,
    pub difficulty: &'static str,
    pub time_complexity: &'static str,
    pub space_complexity: &'static str,
    pub description: &'static str,
    pub examples: Vec<ExampleCase>,
    pub constraints: Vec<&'static str>,
    pub hints: Vec<ProblemHint>,
    pub starter_rust: &'static str,
    pub starter_python: &'static str,
    pub starter_java: &'static str,
    pub starter_typescript: &'static str,
    pub tests: Vec<TestCase>,
}

impl Problem {
    pub fn get_starter_code(&self, lang: SupportedLanguage) -> &'static str {
        match lang {
            SupportedLanguage::Rust => self.starter_rust,
            SupportedLanguage::Python => self.starter_python,
            SupportedLanguage::Java => self.starter_java,
            SupportedLanguage::TypeScript => self.starter_typescript,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConsoleTab {
    Tests,
    Console,
}
