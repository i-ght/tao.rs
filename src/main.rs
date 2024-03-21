use std::collections::HashSet;
use regex::RegexBuilder;

mod tao;

fn main() {
    let mut re_ctor =
        RegexBuilder::new(
            r"(\d+)\n\n(.*?)\n\nBack to Table of Contents"
        );
    let re_ctor =
        re_ctor
        .case_insensitive(true)
        .dot_matches_new_line(true);

    let tao_slice_re = re_ctor.build().unwrap();

    let captures =
        tao_slice_re
            .captures_iter(tao::GIA_FU_FEN_AND_JANE_ENGLISH_TRANSLATION)
            .map(|c| c.extract());
    
    let mut code = Vec::<&str>::with_capacity(81);

    let mut tao: Vec<Vec<&str>> = Vec::with_capacity(81);
    let mut chapters: HashSet<usize> = HashSet::with_capacity(81);

    for (i, (_text, [chapter_str, content])) in captures.enumerate() {
        let chapter = 
            chapter_str
                .parse::<usize>()
                .unwrap();
        assert!(
            chapter != 0 && chapter <= 81 && chapter == i + 1
        );
    
        assert!(chapters.insert(chapter));

        code.insert(i, content);

        let contents: Vec<&str> =
            content
                .split("\n")
                .collect();
        tao.insert(i, contents);
    }

    assert!(chapters.len()==81);

    println!("{}", &code[13])
}