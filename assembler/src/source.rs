use std::path::Path;
use std::io::Read;

#[derive(Debug)]
pub enum SourceError {
    FileNotFound(String),
    ReadError(String),
    InvalidUtf8,
}

impl core::fmt::Display for SourceError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            SourceError::FileNotFound(path) => write!(f, "File not found: {}", path),
            SourceError::ReadError(msg) => write!(f, "Read error: {}", msg),
            SourceError::InvalidUtf8 => write!(f, "Invalid UTF-8 in source"),
        }
    }
}

pub trait Source {
    fn get_source(&self) -> Result<String, SourceError>;
    fn name(&self) -> Option<&str> {
        None
    }
}

impl<T: AsRef<str>> Source for T {
    fn get_source(&self) -> Result<String, SourceError> {
        Ok(self.as_ref().to_string())
    }
    
    fn name(&self) -> Option<&str> {
        None
    }
}

pub struct StringSource {
    content: String,
    name: Option<String>,
}

impl StringSource {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            name: None,
        }
    }
    
    pub fn with_name(content: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            name: Some(name.into()),
        }
    }
}

impl Source for StringSource {
    fn get_source(&self) -> Result<String, SourceError> {
        Ok(self.content.clone())
    }
    
    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

pub struct FileSource {
    path: std::path::PathBuf,
}

impl FileSource {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }
}

impl Source for FileSource {
    fn get_source(&self) -> Result<String, SourceError> {
        let mut file = std::fs::File::open(&self.path)
            .map_err(|e| SourceError::ReadError(e.to_string()))?;
        
        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(|e| SourceError::ReadError(e.to_string()))?;
        
        Ok(content)
    }
    
    fn name(&self) -> Option<&str> {
        self.path.to_str()
    }
}

pub struct LineSource {
    lines: Vec<String>,
    name: Option<String>,
}

impl<const N: usize> From<[&str; N]> for LineSource {
    fn from(lines: [&str; N]) -> Self {
        Self {
            lines: lines.iter().map(|s| s.to_string()).collect(),
            name: None,
        }
    }
}

impl<T: AsRef<str>> From<Vec<T>> for LineSource {
    fn from(lines: Vec<T>) -> Self {
        Self {
            lines: lines.iter().map(|s| s.as_ref().to_string()).collect(),
            name: None,
        }
    }
}

impl<'a> From<&'a [&'a str]> for LineSource {
    fn from(lines: &'a [&'a str]) -> Self {
        Self {
            lines: lines.iter().map(|s| s.to_string()).collect(),
            name: None,
        }
    }
}

impl LineSource {
 
    pub fn new(lines: Vec<String>) -> Self {
        Self { lines, name: None }
    }
    
    pub fn with_name(lines: Vec<String>, name: impl Into<String>) -> Self {
        Self {
            lines,
            name: Some(name.into()),
        }
    }
}

impl Source for LineSource {
    fn get_source(&self) -> Result<String, SourceError> {
        Ok(self.lines.join("\n"))
    }
    
    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}