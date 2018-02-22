pub trait Summarizable {
    fn author_summary(&self) -> String;
    fn summary(&self) -> String {
        // String::from("(Read more...)")
        format!("(Read more from {}...)", self.author_summary())
    }
}

pub fn notify<T: Summarizable>(item: T) {
    println!("Breaking news! {}", item.summary());
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summarizable for NewsArticle {
    fn author_summary(&self) -> String {
        format!("by {} ({})", self.author, self.location)
    }
    fn summary(&self) -> String {
        format!("{}, {}", self.headline, self.author_summary())
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarizable for Tweet {
    fn author_summary(&self) -> String {
        format!("@{}", self.username)
    }
    fn summary(&self) -> String {
        format!("{} {}", self.author_summary(), self.content)
    }
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item
        }
    }
    largest
}

fn main() {
    let tweet = Tweet {
        username: String::from("lencx"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("[1 new tweet] {}", tweet.summary());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in th NHL.")
    };

    print!("New artivle available! {}", article.summary());

    println!("********************");
    notify(tweet);
    notify(article);
    println!("********************");

    let num_list = vec![32, 23, 15, 67, 54, 18];
    let _num = largest(&num_list);
    println!("The largest number is {}", _num);

    let letter = vec!['o', 'm', 'z', 'x', 'i'];
    let _char = largest(&letter);
    println!("The largest char is {}", _char);
}