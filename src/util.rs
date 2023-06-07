use pyu_rust_util as pyu;

pub fn date() {
    pyu::output(pyu::exec("date", "-R"));
}