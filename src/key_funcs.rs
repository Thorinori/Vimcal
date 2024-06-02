use enigo::{Enigo, Direction::{Click, Press, Release}, Key, Settings, Keyboard};

use crate::global_state::GLOBAL_STATE;
pub fn init_binds(){
    //Default Keyboard Bindings TODO: Decide if these remain or let profiles overwrite them
    let tmp = [
        (String::from("delete"),Key::Backspace),
        (String::from("back space"),Key::Backspace),
        (String::from("back"),Key::Backspace),
        (String::from("delete"),Key::Delete),
        (String::from("insert"),Key::Insert),
        (String::from("home"),Key::Home),
        (String::from("end"),Key::End),
        (String::from("print screen"),Key::Print),
        (String::from("scroll lock"),Key::ScrollLock),
        (String::from("num lock"),Key::Numlock),
        (String::from("control"),Key::Control),
        (String::from("alt"),Key::Alt),
        (String::from("shift"),Key::Shift),
        (String::from("meta"),Key::Meta),
        (String::from("mod"),Key::Meta),
        (String::from("caps lock"),Key::CapsLock),
        (String::from("return"),Key::Return),
        (String::from("enter"),Key::Return),
        (String::from("tab"),Key::Tab),
        (String::from("escape"),Key::Escape),
        (String::from("space"),Key::Space),
        (String::from("up"),Key::UpArrow),
        (String::from("page up"),Key::PageUp),
        (String::from("up arrow"),Key::UpArrow),
        (String::from("down"),Key::DownArrow),
        (String::from("page down"),Key::PageDown),
        (String::from("down arrow"),Key::DownArrow),
        (String::from("left"),Key::LeftArrow),
        (String::from("left arrow"),Key::LeftArrow),
        (String::from("right"),Key::RightArrow),
        (String::from("right arrow"),Key::RightArrow),
        (String::from("a"),Key::Unicode('a')),
        (String::from("b"),Key::Unicode('b')),
        (String::from("c"),Key::Unicode('c')),
        (String::from("d"),Key::Unicode('d')),
        (String::from("e"),Key::Unicode('e')),
        (String::from("f"),Key::Unicode('f')),
        (String::from("g"),Key::Unicode('g')),
        (String::from("h"),Key::Unicode('h')),
        (String::from("i"),Key::Unicode('i')),
        (String::from("j"),Key::Unicode('j')),
        (String::from("k"),Key::Unicode('k')),
        (String::from("l"),Key::Unicode('l')),
        (String::from("m"),Key::Unicode('m')),
        (String::from("n"),Key::Unicode('n')),
        (String::from("o"),Key::Unicode('o')),
        (String::from("p"),Key::Unicode('p')),
        (String::from("q"),Key::Unicode('q')),
        (String::from("r"),Key::Unicode('r')),
        (String::from("s"),Key::Unicode('s')),
        (String::from("t"),Key::Unicode('t')),
        (String::from("u"),Key::Unicode('u')),
        (String::from("v"),Key::Unicode('v')),
        (String::from("w"),Key::Unicode('w')),
        (String::from("x"),Key::Unicode('x')),
        (String::from("y"),Key::Unicode('y')),
        (String::from("z"),Key::Unicode('z')),
        (String::from("one"),Key::Unicode('1')),
        (String::from("two"),Key::Unicode('2')),
        (String::from("three"),Key::Unicode('3')),
        (String::from("four"),Key::Unicode('4')),
        (String::from("five"),Key::Unicode('5')),
        (String::from("six"),Key::Unicode('6')),
        (String::from("seven"),Key::Unicode('7')),
        (String::from("eight"),Key::Unicode('8')),
        (String::from("nine"),Key::Unicode('9')),
        (String::from("zero"),Key::Unicode('0')),
        (String::from("f one"),Key::F1),
        (String::from("f two"),Key::F2),
        (String::from("f three"),Key::F3),
        (String::from("f four"),Key::F4),
        (String::from("f five"),Key::F5),
        (String::from("f six"),Key::F6),
        (String::from("f seven"),Key::F7),
        (String::from("f eight"),Key::F8),
        (String::from("f nine"),Key::F9),
        (String::from("f ten"),Key::F10),
        (String::from("f eleven"),Key::F11),
        (String::from("f twelve"),Key::F12),
        (String::from("comma"),Key::Unicode(',')),
        (String::from("left angle bracket"),Key::Unicode('<')),
        (String::from("right angle bracket"),Key::Unicode('>')),
        (String::from("period"),Key::Unicode('.')),
        (String::from("slash"),Key::Unicode('/')),
        (String::from("question mark"),Key::Unicode('?')),
        (String::from("semi colon"),Key::Unicode(';')),
        (String::from("colon"),Key::Unicode(':')),
        (String::from("apostrophe"),Key::Unicode('\'')),
        (String::from("quotation mark"),Key::Unicode('\"')),
        (String::from("backslash"),Key::Unicode('\\')),
        (String::from("pipe"),Key::Unicode('|')),
        (String::from("left brace"),Key::Unicode('[')),
        (String::from("right brace"),Key::Unicode(']')),
        (String::from("left curly brace"),Key::Unicode('{')),
        (String::from("right curly brace"),Key::Unicode('}')),
        (String::from("exlamation point"),Key::Unicode('!')),
        (String::from("at sign"),Key::Unicode('@')),
        (String::from("pound sign"),Key::Unicode('#')),
        (String::from("dollar sign"),Key::Unicode('$')),
        (String::from("percent sign"),Key::Unicode('%')),
        (String::from("caret"),Key::Unicode('^')),
        (String::from("ampersand"),Key::Unicode('&')),
        (String::from("asterisk"),Key::Unicode('*')),
        (String::from("left parentheses"),Key::Unicode('(')),
        (String::from("right parentheses"),Key::Unicode(')')),
        (String::from("dash"),Key::Unicode('-')),
        (String::from("underscore"),Key::Unicode('_')),
        (String::from("plus sign"),Key::Unicode('+')),
        (String::from("equals sign"),Key::Unicode('=')),
        (String::from("tilde"),Key::Unicode('~')),
        (String::from("backtick"),Key::Unicode('`')),
        //NATO Phoentic Alphabet
        (String::from("alpha"),Key::Unicode('a')),
        (String::from("bravo"),Key::Unicode('b')),
        (String::from("charlie"),Key::Unicode('c')),
        (String::from("delta"),Key::Unicode('d')),
        (String::from("echo"),Key::Unicode('e')),
        (String::from("foxtrot"),Key::Unicode('f')),
        (String::from("golf"),Key::Unicode('g')),
        (String::from("hotel"),Key::Unicode('h')),
        (String::from("india"),Key::Unicode('i')),
        (String::from("juliet"),Key::Unicode('j')),
        (String::from("kilo"),Key::Unicode('k')),
        (String::from("lima"),Key::Unicode('l')),
        (String::from("mike"),Key::Unicode('m')),
        (String::from("november"),Key::Unicode('n')),
        (String::from("oscar"),Key::Unicode('o')),
        (String::from("papa"),Key::Unicode('p')),
        (String::from("quebec"),Key::Unicode('q')),
        (String::from("romeo"),Key::Unicode('r')),
        (String::from("sierra"),Key::Unicode('s')),
        (String::from("tango"),Key::Unicode('t')),
        (String::from("uniform"),Key::Unicode('u')),
        (String::from("victory"),Key::Unicode('v')),
        (String::from("whiskey"),Key::Unicode('w')),
        (String::from("x ray"),Key::Unicode('x')),
        (String::from("yankee"),Key::Unicode('y')),
        (String::from("zulu"),Key::Unicode('z')),
        (String::from("niner"),Key::Unicode('9')),
        //Custom Binds Below for now
        (String::from("and"), Key::Unicode('n')),
        (String::from("quinn"), Key::Unicode('q')),
        (String::from("killer"), Key::Unicode('k')),
        (String::from("oh"), Key::Unicode('o')),
        (String::from("jay"), Key::Unicode('j')),
        (String::from("em"), Key::Unicode('m')),
        (String::from("he"), Key::Unicode('e')),
        (String::from("gee"), Key::Unicode('g')),
        (String::from("why"), Key::Unicode('y')),
        (String::from("you"), Key::Unicode('u')),
        (String::from("pee"), Key::Unicode('p')),
        (String::from("tee"), Key::Unicode('t')),
        (String::from("tea"), Key::Unicode('t')),
        (String::from("te"), Key::Unicode('t')),
        (String::from("see"), Key::Unicode('c')),
        (String::from("corn"), Key::Unicode(':')),
        (String::from("colin"), Key::Unicode(':')),
        (String::from("cohen"), Key::Unicode(':')),
        (String::from("coma"), Key::Unicode(',')),
        (String::from("the"), Key::Unicode('d')),
        (String::from("be"), Key::Unicode('b')),
        (String::from("our"), Key::Unicode('r')),
        (String::from("to"), Key::Unicode('2')),
        (String::from("que"), Key::Unicode('q')),
    ];
    for item in tmp{
        GLOBAL_STATE.get().write().unwrap().binds.insert(item.0, item.1);
    }
    

    GLOBAL_STATE.get().write().unwrap().mod_keys = Vec::from([Key::Shift, Key::Control, Key::Alt, Key::Meta]);
}
pub fn press(pressed: &str){
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let binds = &GLOBAL_STATE.get().read().unwrap().binds;
    match binds.get(pressed){
        Some(key) =>{let _ = enigo.key(*key,Click);},
        None=>{
            let words: Vec<&str> = pressed.split_whitespace().collect();
            let mut word_queue: Vec<Key> = Vec::new();
            let mut held: Vec<Key> = Vec::new();
            for word in words{
                match binds.get(word){
                    Some(key) => {word_queue.push(*key)},
                    None => {},
                }
            }
            for key in word_queue{
                if GLOBAL_STATE.get().read().unwrap().mod_keys.contains(&key){
                    let _ = enigo.key(key, Press); 
                    held.push(key);
                }else{
                    let _ = enigo.key(key, Click);
                    for k in &held{
                        let _ = enigo.key(*k, Release);
                    }
                    held.clear();
                    std::thread::sleep(std::time::Duration::from_millis(100));
                }
            }
            for k in &held{
                let _ = enigo.key(*k, Release);
            }
            held.clear();
        },
    }
}
