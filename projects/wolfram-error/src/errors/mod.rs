pub struct WolframError {}

pub enum WolframErrorKind {
    RuntimeError {},
    SyntaxError { message: String, span: FileSpan },
}
