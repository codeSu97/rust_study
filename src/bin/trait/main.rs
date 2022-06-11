/**
 * 特征
 */

pub trait Summary {
    fn summarize_author(&self) -> String;

    // 特征默认实现，允许调用相同特征中的其他方法，哪怕该方法没有默认实现，例如上面的summarize_author
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct Post {
    pub title: String,
    pub author: String,
    pub content: String,
}

impl Summary for Post {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Weibo {
    pub username: String,
    pub content: String,
}

impl Summary for Weibo {
    // 重写
    fn summarize(&self) -> String {
        format!("{} 发表了微博 {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        todo!()
    }
}

fn main() {
    let post = Post {
        title: "Rust语言简介".to_string(),
        author: "Sunface".to_string(),
        content: "Rust棒极了!".to_string(),
    };
    let weibo = Weibo {
        username: "Sunface".to_string(),
        content: "好像微博没有Tweet好用".to_string(),
    };

    println!("{}", post.summarize());
    println!("{}", weibo.summarize());
}
