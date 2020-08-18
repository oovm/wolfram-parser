use crate::{
    ast::WolframStatements,
    parser::codegen::{RootNode, StatementsNode, WolframRule},
};
use diagnostic::{FileCache, FileID};
use std::str::FromStr;
use yggdrasil_rt::YggdrasilError;

impl FromStr for WolframStatements {
    type Err = YggdrasilError<WolframRule>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let root = RootNode::from_str(s)?;
        let mut out = Vec::with_capacity(root.statements.len());
        for x in root.statements {}
    }
}

impl WolframStatements {
    pub fn from_cache(file: FileID, cache: &FileCache) -> Wolfram<WolframStatements, YggdrasilError<WolframRule>> {
        let text = cache.fetch(&file)?;
    }
}

impl StatementsNode {}
