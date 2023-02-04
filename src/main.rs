fn main() {
    let we_load = WebEvent::WELoad(true);

    let click = MouseClick {x : 100, y: 250};

    let we_click = WebEvent::WEClick(click);

    let keys = KeyPress(String::from("Ctrl+"), 'N');

    let we_key = WebEvent::WEKeys(keys);

    println!("{:#?}", we_click);
}

struct Student
{
    name : String,
    level: u8,
    remote: bool
}

#[derive(Debug)]
struct KeyPress(String, char);

#[derive(Debug)]
struct MouseClick{ x: i64, y: i64}

#[derive(Debug)]
enum WebEvent {
    WELoad(bool),
    WEKeys(KeyPress),
    WEClick(MouseClick)
}