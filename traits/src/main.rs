
//A trait defines functionality a particular type has and can share with other types. We can use traits to define shared behavior in an abstract way. We can use trait bounds to specify that a generic type can be any type that has certain behavior.

// Note: Traits are similar to a feature often called interfaces in other languages, although with some differences.

use std::fmt::format;



pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}


pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
// trait summary  is a default action for each trait with summary as implementation
pub trait Summary {
    
    fn summarize_author(&self) -> String ;

    fn summarize(&self) -> String {
        format!("Read more from {}", self.summarize_author())
        // String::from(" Read more btch...{}"),self.summarize_author()

    }
}
// Here we're implementing Summary for NewsArticle  and defining a custom summarize action
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    fn summarize_author(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}
// here we're implementing Summary for Tweet but we're not going to implement a custom summarize action 
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("{}, by {} ", self.content, self.username)
    }
}


fn main() {

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    
println!("Tweet{}   " , tweet.summarize());
print!("article{} " , article.summarize());
}
