// Estructuras de datos
#[derive(Debug, PartialEq, Clone)]
pub struct Fragment {
    pub path: Path,
    pub range: Option<Range>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Path {
    pub local_path: Box<LocalPath>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Range {
    pub start: LocalPath,
    pub end: LocalPath,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LocalPath {
    pub steps: Vec<Step>,
    pub end: Option<Box<EndOfPath>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum EndOfPath {
    RedirectedPath(Box<RedirectedPath>),
    Offset(Offset),
}

#[derive(Debug, PartialEq, Clone)]
pub struct RedirectedPath {
    pub offset: Option<Offset>,
    pub path: Option<Box<Path>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Step {
    pub integer: String,
    pub assertion: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Offset {
    SimpleOffset(String),
    OffsetWithReference(String, String),
    IdAssertion(String, Option<(String, String)>),
}


impl Fragment {
    pub fn new(path: Path, range: Option<Range>) -> Self {
        Fragment { path, range }
    }
}

impl Path {
    pub fn new( local_path: LocalPath) -> Self {
        Path { local_path: Box::new(local_path) }
    }
}

impl Range {
    pub fn new(start: LocalPath, end: LocalPath) -> Self {
        Range { start, end }
    }
}

impl LocalPath {
    pub fn new(steps: Vec<Step>, end: Option<EndOfPath>) -> Self {
        LocalPath { steps, end: end.map(Box::new) }
    }
}

impl EndOfPath {
    pub fn redirected_path(redirected_path: RedirectedPath) -> Self {
        EndOfPath::RedirectedPath(Box::new(redirected_path))
    }

    pub fn offset(offset: Offset) -> Self {
        EndOfPath::Offset(offset)
    }
}

impl RedirectedPath {
    pub fn new(offset: Option<Offset>, path: Option<Path>) -> Self {
        RedirectedPath { offset, path: path.map(Box::new) }
    }
}

impl Step {
    pub fn new(integer: String, assertion: Option<String>) -> Self {
        Step { integer, assertion }
    }
}

impl Offset {
    pub fn simple_offset(value: String) -> Self {
        Offset::SimpleOffset(value)
    }

    pub fn offset_with_reference(val1: String, val2: String) -> Self {
        Offset::OffsetWithReference(val1, val2)
    }

    pub fn id_assertion(val: String, reference: Option<(String, String)>) -> Self {
        Offset::IdAssertion(val, reference)
    }
}
