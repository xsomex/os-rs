use crate::memory::vec::HeaplessVec as Vec;

use super::{Char, Font};
use crate::{add_char, b_vec};

const F: bool = true;
const O: bool = false;

pub const MONOSPACE: Font = {
    let mut font = Font {
        chars: Vec::new(),
        bytes: Vec::new(),
        width: 8,
        height: 16,
    };

    add_char!(
        font,
        ' ',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        '!',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        '"',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, F, F, O, O, F, F, O),
            b_vec!(O, F, F, O, O, F, F, O),
            b_vec!(O, F, F, O, O, F, F, O),
            b_vec!(O, F, F, O, O, F, F, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        '#',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, F, F, O, F, F, O, O),
            b_vec!(O, F, F, O, F, F, O, O),
            b_vec!(F, F, F, F, F, F, F, O),
            b_vec!(O, F, F, O, F, F, O, O),
            b_vec!(O, F, F, O, F, F, O, O),
            b_vec!(O, F, F, O, F, F, O, O),
            b_vec!(O, F, F, O, F, F, O, O),
            b_vec!(F, F, F, F, F, F, F, O),
            b_vec!(O, F, F, O, F, F, O, O),
            b_vec!(O, F, F, O, F, F, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        '$',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, F, O, O, O, O),
            b_vec!(O, F, F, F, F, F, F, O),
            b_vec!(F, F, O, F, O, O, O, O),
            b_vec!(F, F, O, F, O, O, O, O),
            b_vec!(F, F, O, F, O, O, O, O),
            b_vec!(O, F, F, F, F, F, O, O),
            b_vec!(O, O, O, F, O, F, F, O),
            b_vec!(O, O, O, F, O, F, F, O),
            b_vec!(O, O, O, F, O, F, F, O),
            b_vec!(O, O, O, F, O, F, F, O),
            b_vec!(F, F, F, F, F, F, O, O),
            b_vec!(O, O, O, F, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        '%',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, F, F, O),
            b_vec!(O, F, F, O, O, F, F, O),
            b_vec!(O, F, F, O, F, F, O, O),
            b_vec!(O, O, O, O, F, F, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, O, F, F, O, F, F, O),
            b_vec!(O, F, F, O, O, F, F, O),
            b_vec!(O, F, F, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        '&',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, F, F, F, O, O, O),
            b_vec!(O, F, F, O, F, F, O, O),
            b_vec!(O, F, F, O, F, F, O, O),
            b_vec!(O, F, F, O, F, F, O, O),
            b_vec!(O, O, F, F, F, O, O, O),
            b_vec!(O, F, F, F, O, O, O, O),
            b_vec!(F, F, O, F, F, O, F, O),
            b_vec!(F, F, O, O, F, F, O, O),
            b_vec!(F, F, O, O, F, F, O, O),
            b_vec!(O, F, F, F, F, O, F, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        '\'',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        '(',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, F, F, F, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, F, F, O, O, O, O, O),
            b_vec!(O, F, F, O, O, O, O, O),
            b_vec!(O, F, F, O, O, O, O, O),
            b_vec!(O, F, F, O, O, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, O, F, F, F, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        ')',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, F, F, F, O, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, O, F, F, O, O),
            b_vec!(O, O, O, O, F, F, O, O),
            b_vec!(O, O, O, O, O, F, F, O),
            b_vec!(O, O, O, O, O, F, F, O),
            b_vec!(O, O, O, O, O, F, F, O),
            b_vec!(O, O, O, O, O, F, F, O),
            b_vec!(O, O, O, O, F, F, O, O),
            b_vec!(O, O, O, O, F, F, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, F, F, F, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        '*',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, F, F, O, O, F, F, O),
            b_vec!(O, O, F, F, F, F, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(F, F, F, F, F, F, F, F),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, F, F, F, F, O, O),
            b_vec!(O, F, F, O, O, F, F, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        '+',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, F, F, F, F, F, F, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        ',',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        '-',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, F, F, F, F, F, F, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        '.',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        '/',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, F, F, O),
            b_vec!(O, O, O, O, O, F, F, O),
            b_vec!(O, O, O, O, F, F, O, O),
            b_vec!(O, O, O, O, F, F, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, F, F, O, O, O, O, O),
            b_vec!(O, F, F, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        '0',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, F, F, F, F, F, O, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, F, F, F, O),
            b_vec!(F, F, O, F, F, F, F, O),
            b_vec!(F, F, F, F, O, F, F, O),
            b_vec!(F, F, F, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(O, F, F, F, F, F, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        '1',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, F, F, F, O, O, O),
            b_vec!(O, F, F, F, F, O, O, O),
            b_vec!(O, F, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, F, F, F, F, F, F, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        '2',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, F, F, F, F, F, O, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(O, O, O, O, O, F, F, O),
            b_vec!(O, O, O, O, O, F, F, O),
            b_vec!(O, O, O, O, F, F, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, F, F, O, O, O, O, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, F, F, F, F, F, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        '3',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, F, F, F, F, F, O, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(O, O, O, O, O, F, F, O),
            b_vec!(O, O, O, O, O, F, F, O),
            b_vec!(O, O, F, F, F, F, O, O),
            b_vec!(O, O, O, O, O, F, F, O),
            b_vec!(O, O, O, O, O, F, F, O),
            b_vec!(O, O, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(O, O, F, F, F, F, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        '4',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, F, F, O, O),
            b_vec!(F, F, O, O, F, F, O, O),
            b_vec!(F, F, O, O, F, F, O, O),
            b_vec!(F, F, O, O, F, F, O, O),
            b_vec!(F, F, F, F, F, F, F, O),
            b_vec!(O, O, O, O, F, F, O, O),
            b_vec!(O, O, O, O, F, F, O, O),
            b_vec!(O, O, O, O, F, F, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        '5',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(F, F, F, F, F, F, F, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, F, F, F, F, O, O),
            b_vec!(O, O, O, O, O, F, F, O),
            b_vec!(O, O, O, O, O, F, F, O),
            b_vec!(O, O, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(O, F, F, F, F, F, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        '6',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, F, F, F, F, F, O, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, F, F, F, F, O, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(O, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(O, F, F, F, F, F, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        '7',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(F, F, F, F, F, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(O, O, O, O, O, F, F, O),
            b_vec!(O, O, O, O, O, F, F, O),
            b_vec!(O, O, O, O, F, F, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        '8',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, F, F, F, F, F, O, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(O, F, F, F, F, F, O, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(O, F, F, F, F, F, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        '9',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, F, F, F, F, F, O, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(O, F, F, F, F, F, F, O),
            b_vec!(O, O, O, O, O, F, F, O),
            b_vec!(O, O, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(O, F, F, F, F, F, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        ':',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        ';',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        '<',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, F, F, O),
            b_vec!(O, O, O, O, F, F, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, F, F, O, O, O, O, O),
            b_vec!(O, F, F, O, O, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, O, F, F, O, O),
            b_vec!(O, O, O, O, O, F, F, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        '=',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, F, F, F, F, F, F, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, F, F, F, F, F, F, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        '>',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, F, F, O, O, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, O, F, F, O, O),
            b_vec!(O, O, O, O, O, F, F, O),
            b_vec!(O, O, O, O, O, F, F, O),
            b_vec!(O, O, O, O, F, F, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, F, F, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        '?',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, F, F, F, F, F, O, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(O, O, O, O, O, F, F, O),
            b_vec!(O, O, O, O, F, F, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        '@',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, F, F, F, F, F, O, O),
            b_vec!(F, F, O, O, O, O, F, O),
            b_vec!(F, F, O, F, F, O, F, O),
            b_vec!(F, F, O, F, F, O, F, O),
            b_vec!(F, F, O, F, F, O, F, O),
            b_vec!(F, F, O, F, F, O, F, O),
            b_vec!(F, F, O, F, F, F, F, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(O, F, F, F, F, F, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        'A',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, F, F, F, F, F, O, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, F, F, F, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        'B',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(F, F, F, F, F, F, O, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, F, F, F, F, O, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, F, F, F, F, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        'C',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, F, F, F, F, F, F, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(O, F, F, F, F, F, F, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        'D',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(F, F, F, F, F, F, O, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, F, F, F, F, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        'E',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, F, F, F, F, F, F, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, F, F, F, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(O, F, F, F, F, F, F, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        'F',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, F, F, F, F, F, F, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, F, F, F, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        'G',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, F, F, F, F, F, F, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, F, F, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(O, F, F, F, F, F, F, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        'H',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, F, F, F, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        'I',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, F, F, F, F, F, F, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, F, F, F, F, F, F, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        'J',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, F, F, F, F, F, F, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(F, F, F, F, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        'K',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, F, F, O, O),
            b_vec!(F, F, F, F, F, O, O, O),
            b_vec!(F, F, O, O, F, F, O, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        'L',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(O, F, F, F, F, F, F, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        'M',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, F, O, F, F, F, O),
            b_vec!(F, F, F, F, F, F, F, O),
            b_vec!(F, F, O, F, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        'N',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, F, O, O, F, F, O),
            b_vec!(F, F, F, O, O, F, F, O),
            b_vec!(F, F, O, F, O, F, F, O),
            b_vec!(F, F, O, F, O, F, F, O),
            b_vec!(F, F, O, O, F, F, F, O),
            b_vec!(F, F, O, O, F, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        'O',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, F, F, F, F, F, O, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(O, F, F, F, F, F, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        'P',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(F, F, F, F, F, F, O, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, F, F, F, F, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        'Q',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, F, F, F, F, F, O, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, F, O, F, F, O),
            b_vec!(F, F, O, F, O, F, F, O),
            b_vec!(O, F, F, F, F, F, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, O, F, F, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        'R',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(F, F, F, F, F, F, O, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, F, F, F, F, O, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        'S',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, F, F, F, F, F, F, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(O, F, F, F, F, F, O, O),
            b_vec!(O, O, O, O, O, F, F, O),
            b_vec!(O, O, O, O, O, F, F, O),
            b_vec!(O, O, O, O, O, F, F, O),
            b_vec!(O, O, O, O, O, F, F, O),
            b_vec!(F, F, F, F, F, F, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        'T',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(F, F, F, F, F, F, F, F),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        'U',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(O, F, F, F, F, F, F, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        'V',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(O, F, F, O, F, F, O, O),
            b_vec!(O, O, F, F, F, O, O, O),
            b_vec!(O, O, O, F, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        'W',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, F, O, F, F, O),
            b_vec!(F, F, F, F, F, F, F, O),
            b_vec!(F, F, F, O, F, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        'X',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(O, F, F, O, F, F, O, O),
            b_vec!(O, O, F, F, F, O, O, O),
            b_vec!(O, F, F, O, F, F, O, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        'Y',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(O, F, F, F, F, F, F, O),
            b_vec!(O, O, O, O, O, F, F, O),
            b_vec!(O, O, O, O, O, F, F, O),
            b_vec!(O, O, O, O, O, F, F, O),
            b_vec!(O, O, O, O, O, F, F, O),
            b_vec!(F, F, F, F, F, F, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        'Z',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(F, F, F, F, F, F, F, O),
            b_vec!(O, O, O, O, O, F, F, O),
            b_vec!(O, O, O, O, O, F, F, O),
            b_vec!(O, O, O, O, F, F, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, F, F, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, F, F, F, F, F, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        '[',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, F, F, F, F, F, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, O, F, F, F, F, F, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        '\\',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(O, F, F, O, O, O, O, O),
            b_vec!(O, F, F, O, O, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, O, F, F, O, O),
            b_vec!(O, O, O, O, F, F, O, O),
            b_vec!(O, O, O, O, O, F, F, O),
            b_vec!(O, O, O, O, O, F, F, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        ' ',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, F, F, F, F, F, O, O),
            b_vec!(O, O, O, O, F, F, O, O),
            b_vec!(O, O, O, O, F, F, O, O),
            b_vec!(O, O, O, O, F, F, O, O),
            b_vec!(O, O, O, O, F, F, O, O),
            b_vec!(O, O, O, O, F, F, O, O),
            b_vec!(O, O, O, O, F, F, O, O),
            b_vec!(O, O, O, O, F, F, O, O),
            b_vec!(O, O, O, O, F, F, O, O),
            b_vec!(O, O, O, O, F, F, O, O),
            b_vec!(O, O, O, O, F, F, O, O),
            b_vec!(O, F, F, F, F, F, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        '^',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, F, O, O, O, O),
            b_vec!(O, O, F, F, F, O, O, O),
            b_vec!(O, F, F, O, F, F, O, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        '_',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(F, F, F, F, F, F, F, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        '`',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, O, F, F, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        'a',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, F, F, F, F, F, O, O),
            b_vec!(O, O, O, O, O, F, F, O),
            b_vec!(O, F, F, F, F, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(O, F, F, F, F, F, F, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        'b',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, F, F, F, F, O, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, F, F, F, F, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        'c',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, F, F, F, F, F, F, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(O, F, F, F, F, F, F, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        'd',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, F, F, O),
            b_vec!(O, O, O, O, O, F, F, O),
            b_vec!(O, O, O, O, O, F, F, O),
            b_vec!(O, F, F, F, F, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(O, F, F, F, F, F, F, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        'e',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, F, F, F, F, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, F, F, F, F, F, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(O, F, F, F, F, F, F, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        'f',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, F, F, F, F, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, F, F, F, F, F, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        'g',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, F, F, F, F, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(O, F, F, F, F, F, O, O),
            b_vec!(O, O, O, O, O, F, F, O),
            b_vec!(O, O, O, O, O, F, F, O),
            b_vec!(F, F, F, F, F, F, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        'h',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, F, F, F, F, O, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        'i',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, F, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, F, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        'j',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, F, F, F, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        'k',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, F, F, O, O),
            b_vec!(F, F, O, F, F, O, O, O),
            b_vec!(F, F, F, F, O, O, O, O),
            b_vec!(F, F, F, F, O, O, O, O),
            b_vec!(F, F, O, F, F, O, O, O),
            b_vec!(F, F, O, O, F, F, O, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        'l',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, O, O, F, F, F, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        'm',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(F, F, F, O, F, F, O, O),
            b_vec!(F, F, O, F, O, F, F, O),
            b_vec!(F, F, O, F, O, F, F, O),
            b_vec!(F, F, O, F, O, F, F, O),
            b_vec!(F, F, O, F, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        'n',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(F, F, F, F, F, F, O, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        'o',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, F, F, F, F, F, O, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(O, F, F, F, F, F, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        'p',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(F, F, F, F, F, F, O, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, F, F, F, F, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        'q',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, F, F, F, F, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(O, F, F, F, F, F, F, O),
            b_vec!(O, O, O, O, O, F, F, O),
            b_vec!(O, O, O, O, O, F, F, O),
            b_vec!(O, O, O, O, O, F, F, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        'r',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, F, F, F, F, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        's',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, F, F, F, F, F, F, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, O, O, O),
            b_vec!(O, F, F, F, F, F, O, O),
            b_vec!(O, O, O, O, O, F, F, O),
            b_vec!(O, O, O, O, O, F, F, O),
            b_vec!(F, F, F, F, F, F, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        't',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, F, F, F, F, F, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, O, O, F, F, F, F, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        'u',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(O, F, F, F, F, F, F, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        'v',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(O, F, F, O, F, F, O, O),
            b_vec!(O, O, F, F, F, O, O, O),
            b_vec!(O, O, O, F, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        'w',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, F, O, F, F, O),
            b_vec!(F, F, O, F, O, F, F, O),
            b_vec!(F, F, O, F, O, F, F, O),
            b_vec!(F, F, O, F, O, F, F, O),
            b_vec!(O, F, F, O, F, F, F, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        'x',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(O, F, F, O, F, F, O, O),
            b_vec!(O, O, F, F, F, O, O, O),
            b_vec!(O, O, F, F, F, O, O, O),
            b_vec!(O, F, F, O, F, F, O, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        'y',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(F, F, O, O, O, F, F, O),
            b_vec!(O, F, F, F, F, F, F, O),
            b_vec!(O, O, O, O, O, F, F, O),
            b_vec!(O, O, O, O, O, F, F, O),
            b_vec!(F, F, F, F, F, F, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        'z',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(F, F, F, F, F, F, F, O),
            b_vec!(O, O, O, O, F, F, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, F, F, O, O, O, O),
            b_vec!(O, F, F, O, O, O, O, O),
            b_vec!(F, F, F, F, F, F, F, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        '{',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, F, F, F, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, F, F, F, O, O, O, O),
            b_vec!(O, F, F, F, O, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, O, F, F, F, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        '|',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        '}',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, F, F, F, O, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, O, F, F, F, O),
            b_vec!(O, O, O, O, F, F, F, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, O, O, F, F, O, O, O),
            b_vec!(O, F, F, F, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );
    add_char!(
        font,
        '~',
        (8, 16),
        (0, 0),
        b_vec!(
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, F, F, O, O, F, O),
            b_vec!(O, F, F, F, F, F, F, O),
            b_vec!(O, F, O, O, F, F, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O),
            b_vec!(O, O, O, O, O, O, O, O)
        )
    );

    font
};
