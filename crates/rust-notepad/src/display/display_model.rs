
#[derive(Debug, PartialEq, Clone, Default)]
pub struct DisplayModel {
    pub title: String,
    pub body: String,
    pub status: String,
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