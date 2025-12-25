mod content;
use content::media::{Media, describe_into, create_buf}; 
use content::catalog::Catalog;

fn main() {
    let book = Media::new_book("The Art of War", "Sun Tzu");
    let audiobook = Media::new_audiobook("The 48 Laws of Power");
    let movie = Media::new_movie("Batman", "Christopher Nolan");
    let podcast = Media::new_podcast(1000);
    let mut library = Catalog::new();
    let mut query;

    // Check libary created 
    if library.is_empty() { println!("Library exists."); }

    book.print_media();
    audiobook.print_media();
    movie.print_media();
    podcast.print_media();

    library.add(book);
    query = library.get_by_index(0);
    println!("Media query: {:?}", query);

    library.add(movie);
    query = library.get_by_index(1);
    println!("Media query: {:?}", query);

    println!("Size of library: {:?}", library.len());

    // Get a description
    let mut buf = create_buf::<64>();
    describe_into(&audiobook, &mut buf);
    println!("Audiobook description: {:?}", buf);

}
