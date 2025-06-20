#[macro_use]
mod macros;

mod builder;
mod tokenization;
mod tokens;

Contextable! {
    struct UserTest {
        name: String,
        age: i32,
        role: i32,
    }
}
fn main() {
    let templ = "Sveikas, {{ UserTest.name }}! you are {% if UserTest.age >= 18 %} a man {%else%} a boy {%endif%}. {% if UserTest.role == 1 %} You are the one who knocks {%endif%}";
    let user = UserTest {
        name: "lazymonad".into(),
        age: 22,
        role: 0,
    };

    let htdl = builder::Builder {
        context: user.flatten(),
        content: String::from(templ),
    };

    println!("Build: {}", htdl.build());
}
