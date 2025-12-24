use core::fmt;
use heapless::{String, Vec};

#[derive(Debug, PartialEq, Eq)]
enum Media {
    Book { title: &'static str, author: &'static str },
    Movie { title: &'static str, director: &'static str },
    Audiobook { title: &'static str },
    Podcast { episode_number: u32 },
    Placeholder,
}

impl Media {
    fn description<W: fmt::Write>(&self, w: &mut W)  -> fmt::Result { // sink pattern
        match self {
            Media::Book { title, author } => 
                write!(w, "Book: {} {}", title, author),
            Media::Movie { title, director } =>
                write!(w, "Movie: {} {}", title, director),
            Media::Audiobook { title } =>
                write!(w, "Audiobook: {}", title),
            Media::Podcast { episode_number } =>
                write!(w, "Podcast: {}", episode_number),
            Media::Placeholder =>
                write!(w, "Placeholder {}", ""),
        }
    }

    fn print_media(&self) {       // borrows self.? and prints
        println!("{:#?}", self);
    }

}

struct Catalog {
    media: Vec<Media, 20>,  
}

impl Catalog {
    fn new() -> Self {
        Self { media: Vec::new() }
    }

    fn try_add(&mut self, m: Media) -> Result<(), Media> {
        self.media.push(m)
    }

    fn add(&mut self, m: Media) {
        self.try_add(m).expect("Catalog capacity exceeded");
    }

    fn try_get_by_index(&self, index: usize) -> Option<&Media> {
        self.media.get(index)
    }

    fn get_by_index(&self, index: usize) -> &Media {
        self.try_get_by_index(index)
            .expect("Catalog index out of range")
    }

    fn get_by_index_variant<'a>(&'a self, index: usize) -> MightHaveAValue<'a> {
        if index < self.media.len() {
            // Good! We have something to return
            MightHaveAValue::ThereIsAValue(&self.media[index])
        } else {
            // Bad! We don't have anything to return!!!
            MightHaveAValue::NoValueAvailable
        }
    }
}

// Helpers
#[derive(Debug)]
enum MightHaveAValue<'a> { // Variant of Option to return if something is or isn't there 
    ThereIsAValue(&'a Media), // Ref to some media thing; means we have a value
    NoValueAvailable,
} // we use the "'a" for a lifetime annotation 

fn create_buf<const N: usize>() -> heapless::String::<N> {
    heapless::String::<N>::new()
}

fn describe_into<const N: usize>(
    m: &Media,
    buf: &mut heapless::String<N>,
) {
    // Policy decision lives HERE
    let _ = m.description(buf);
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audiobook_description_formats() {
        let audiobook = Media::Audiobook { title: "The 48 Laws of Power" };

        let mut buf = create_buf::<64>();
        describe_into(&audiobook, &mut buf);

        assert_eq!(buf.as_str(), "Audiobook: The 48 Laws of Power");
    }

    #[test]
    fn test_init_catalog() {
        let catalog = Catalog::new(); 

        assert_eq!(catalog.media.is_empty(), true);
    }

    #[test]
    fn test_libary_add() {
        let mut catalog = Catalog::new();

        let audiobook = Media::Audiobook { title: "New Audiobook" };
        catalog.add(audiobook); // moves audiobook ownership to Catalog

        let query = catalog.media.get(0); 
        let expect = Media::Audiobook { title: "New Audiobook" };
        assert_eq!(query, Some(&expect));
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

}

fn main() {
    let mut audiobook = Media::Audiobook { 
        title: "The 48 Laws of Power",
    };

    let mut book = Media::Book {
        title: "The Art of War",
        author: "Sun Tzu",
    };

    let mut movie = Media::Movie {
        title: "Batman", 
        director: "Christopher Nolan",
    };

    audiobook.print_media();
    book.print_media();
    audiobook.print_media();

    let mut library = Catalog::new();
    library.add(book);
    let mut query = library.media.get(0);
    println!("Media query: {:?}", query);

    library.add(movie);
    query = library.media.get(1);
    println!("Media query: {:?}", query);

}
