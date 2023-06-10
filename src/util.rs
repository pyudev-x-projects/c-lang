use pyu_rust_util as pyu;

pub fn date() {
    pyu::output(pyu::exec("date", "-R"));
}

pub fn clear() {
   pyu::output(pyu::exec("clear", "-x")); 
}

pub fn lorem() {
    println!("Lorem ipsum dolor sit amet, consectetur adipisicing elit. (Latin)");
    println!("It is very important for the customer to pay attention to the undergraduate process. (English - Translated)");
}

pub fn newl() {
    println!("");
}