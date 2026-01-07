mod content;
use { content::media::Media, content::catalog::Catalog };

fn print_media(media: &Media) {
    println!("{:#?}", media);
}

fn main() {
    let audiobook = Media::Audiobook { title: String::from("Buyology") };
    let good_movie = Media::Movie { title: String::from("The Shawshank Redemption"), director: String::from("Frank Darabont") };
    let bad_movie = Media::Book { title: String::from("You Can Win"), author: String::from("Shiv Khera") };
    let placeholder = Media::Placeholder;
    let podcast = Media::Podcast(1);

    // println!("{}", audiobook.description());
    // println!("{}", good_movie.description());
    // println!("{}", bad_movie.description());

    // print_media(audiobook);
    // print_media(good_movie);
    // print_media(bad_movie);

    let mut catalog = Catalog::new();

    catalog.add(audiobook);
    catalog.add(good_movie);
    catalog.add(bad_movie);
    catalog.add(placeholder);
    catalog.add(podcast);

    let item = catalog.get_by_index(0);
    print_media(item.unwrap());
    
    let item3 = catalog.get_by_index(6);
    let placeholder = Media::Placeholder;
    print_media(item3.unwrap_or(&placeholder));

    let item2 = catalog.get_by_index(10);
    print_media(item2.expect("expected to have an item here"));
    // match catalog.get_by_index(100) {
    //     Some(value) => {
    //         print_media(value);
    //     },
    //     None => {
    //         println!("There is no value");
    //     }
    // }

    // if let Some(value) = catalog.get_by_index(10) {
    //     print_media(value);
    // } else {
    //     println!("There is no value");
    // }
}
