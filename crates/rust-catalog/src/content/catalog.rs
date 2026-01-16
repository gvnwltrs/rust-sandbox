use super::media::Media;
use heapless::Vec;

pub struct Catalog {
    media: Vec<Media, 20>,  
}

impl Catalog {
    pub fn new() -> Self {
        Self { media: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.media.len()
    }

    pub fn is_empty(&self) -> bool {
        self.media.is_empty()
    }

    pub fn try_add(&mut self, m: Media) -> Result<(), Media> {
        self.media.push(m)
    }

    pub fn add(&mut self, m: Media) {
        self.try_add(m).expect("Catalog capacity exceeded");
    }

    pub fn try_get_by_index(&self, index: usize) -> Option<&Media> {
        self.media.get(index)
    }

    pub fn get_by_index(&self, index: usize) -> &Media {
        self.try_get_by_index(index)
            .expect("Catalog index out of range")
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_catalog() {
        let catalog = Catalog::new(); 

        assert_eq!(catalog.is_empty(), true);
    }

    #[test]
    fn test_libary_add() {
        let mut catalog = Catalog::new();

        let audiobook = Media::Audiobook { title: "New Audiobook" };
        catalog.add(audiobook); // moves audiobook ownership to Catalog

        let query = catalog.get_by_index(0); 
        let expect = Media::Audiobook { title: "New Audiobook" };
        assert_eq!(query, &expect);
    }

    #[test]
    fn test_get_by_index() {
        let mut catalog = Catalog::new();
        let audiobook = Media::Audiobook { title: "New Book" }; 
        catalog.add(audiobook); 

        let query = catalog.get_by_index(0);
        let expect = Media::Audiobook { title: "New Book" }; 
        assert_eq!(query, &expect);
    }

    #[test]
    fn test_build_new_media() {
        let book = Media::new_book("Fear and Loathing in Las Vegas", "Hunter S. Thompson");
        let expect = Media::Book { 
            title: "Fear and Loathing in Las Vegas", 
            author: "Hunter S. Thompson"
        };

        assert_eq!(book, expect);
    }

}