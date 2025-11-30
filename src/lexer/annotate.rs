#[derive(Debug, Clone, PartialEq)]
pub enum Annotation {
    PartialEval,
    NoPartialEval,
    Static,
    Dynamic,
}

impl Annotation {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "pe"     => Some(Self::PartialEval),
            "nope"   => Some(Self::NoPartialEval),
            "static" => Some(Self::Static),
            "dynamic"=> Some(Self::Dynamic),
            _ => None,
        }
    }
}
