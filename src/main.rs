use tao;
use rand::Rng;

fn main() {
    let code = 
        tao::slice(tao::gfje_engtext::GIA_FU_FEN_AND_JANE_ENGLISH_TRANSLATION);

    let z = rand::thread_rng().gen_range(0..=code.len());
    let tao = &code[z];
    println!("{:#?}", ((z+1), tao));
}       