use tao::slice;

mod gfje_engtext;

fn main() {
    let code = 
        slice(gfje_engtext::GIA_FU_FEN_AND_JANE_ENGLISH_TRANSLATION);
    
    println!("{:#?}", &code[13])
}