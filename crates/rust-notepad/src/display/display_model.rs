
#[derive(Debug, PartialEq, Clone, Default)]
pub struct DisplayModel {
    pub title: String,
    pub body: String,
    pub status: String,
}

#[derive(Debug, Clone)]
pub enum GuiInput {
    SetBody(String),
    SaveRequested,
    OpenRequested,
    ClearRequested,
}

#[cfg(test)]
mod tests {

    #[allow(unused)]
    use super::*;

    #[test]
    fn smoke_test() {
        assert!(true);
    }
} 