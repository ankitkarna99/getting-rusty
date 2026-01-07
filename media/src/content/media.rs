#[derive(Debug)]
pub enum Media {
    Book {title: String, author: String},
    Movie {title: String, director: String},
    Audiobook {title: String},
    Podcast(u32),
    Placeholder,
}

impl Media {
    pub fn description(&self) -> String {
        // if let Media::Book { title, author } = self {
        //     return format!("{} by {}", title, author);
        // } else if let Media::Movie { title, director } = self {
        //     return format!("{} directed by {}", title, director);
        // } else if let Media::Audiobook { title } = self {
        //     return format!("{} is an audiobook", title);
        // } else {
        //     return String::from("");
        // }

        match self {
            Media::Book { title, author } => {
                return format!("{} by {}", title, author)
            },
            Media::Movie { title, director } => {
                return format!("{} directed by {}", title, director)
            },
            Media::Audiobook { title } => {
                return format!("{} is an audiobook", title)
            },
            Media::Podcast(epsiode_number) => {
                return format!("{} is an podcast", epsiode_number)
            },
            Media::Placeholder => {
                return String::from("Placeholder")
            },
        }
    }
}