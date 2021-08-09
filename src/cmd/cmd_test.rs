use std::path::PathBuf;

use super::Renamee;
use crate::engine;

#[test]
fn new_name() {
    let dummy = PathBuf::from("/bin/lmao");
    let re = engine::compile("lmao season 1 episode @N@.mp4").unwrap();

    let tests = (1..11)
        .map(|n| format!("lmao season 1 episode {}.mp4", n))
        .map(|s| {
            let indices = re
                .captures(&s)
                .expect("the regex didn't match")
                .iter()
                .skip(1)
                .filter_map(|o| o.map(|c| (c.start(), c.end())))
                .collect::<Vec<_>>();

            Renamee {
                path: &dummy,
                old: s,
                indices,
            }
        });

    for (i, test) in tests.into_iter().enumerate() {
        let new_name = test.new_name(&[2]);
        assert_eq!(
            format!("lmao season 1 episode {:0>2}.mp4", i + 1),
            new_name,
            "the new name didn't match the old name",
        );
    }
}
