use core::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Media {
    Book { title: &'static str, author: &'static str },
    Movie { title: &'static str, director: &'static str },
    Audiobook { title: &'static str },
    Podcast { episode_number: u32 },
}

impl Media {
    // TODO: Implement way to create ANY media with new() 

    // Naive approach to TODO
    pub fn new_book(title: &'static str, author: &'static str) -> Self {
        Self::Book { title, author }
    }

    pub fn new_movie(title: &'static str, director: &'static str) -> Self {
        Self::Movie { title, director }
    }

    pub fn new_audiobook(title: &'static str) -> Self {
        Self::Audiobook { title }
    }

    pub fn new_podcast(episode_number: u32) -> Self {
        Self::Podcast { episode_number }
    }

    pub fn description<W: fmt::Write>(&self, w: &mut W)  -> fmt::Result { // sink pattern
        match self {
            Media::Book { title, author } => 
                write!(w, "Book: {} {}", title, author),
            Media::Movie { title, director } =>
                write!(w, "Movie: {} {}", title, director),
            Media::Audiobook { title } =>
                write!(w, "Audiobook: {}", title),
            Media::Podcast { episode_number } =>
                write!(w, "Podcast: {}", episode_number),
        }
    }

    pub fn print_media(&self) {       // borrows self.? and prints
        println!("{:#?}", self);
    }

}

// Helpers
pub fn create_buf<const N: usize>() -> heapless::String::<N> {
    heapless::String::<N>::new()
}

pub fn describe_into<const N: usize>(
    m: &Media,
    buf: &mut heapless::String<N>,
) {
    // Policy decision lives HERE
    let _ = m.description(buf);
}

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
}