pub struct Movables {
    pub null:               [(i8, i8); 0],
    pub pawn_black:         [(i8, i8); 1],
    pub knight_black:       [(i8, i8); 2],
    pub silver_black:       [(i8, i8); 5],
    pub gold_black:         [(i8, i8); 6],
    pub king_black:         [(i8, i8); 8],
    pub lance_black:        [(i8, i8); 8],
    pub bishop_black:       [[(i8, i8); 8]; 4],
    pub rook_black:         [[(i8, i8); 8]; 4],
    pub normal_horse_black:        [(i8, i8); 4],
    pub normal_dragon_black:       [(i8, i8); 4],
    pub pawn_white:         [(i8, i8); 1],
    pub knight_white:       [(i8, i8); 2],
    pub silver_white:       [(i8, i8); 5],
    pub gold_white:         [(i8, i8); 6],
    pub king_white:         [(i8, i8); 8],
    pub lance_white:        [(i8, i8); 8],
    pub bishop_white:       [[(i8, i8); 8]; 4],
    pub rook_white:         [[(i8, i8); 8]; 4],
    pub normal_horse_white:        [(i8, i8); 4],
    pub normal_dragon_white:       [(i8, i8); 4],
}

pub const MOVABLES : Movables = Movables {
    null: [],
    // normal
    pawn_black: [(-1, 0)],
    knight_black: [(-2, -1), (-2, 1)],
    silver_black: [(-1, -1), (-1, 0), (-1, 1), (1, -1), (1, 1)],
    gold_black: [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, 0)],
    king_black: [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)],
    normal_horse_black: [(1, 0), (0, -1), (0, 1), (-1, 0)],
    normal_dragon_black: [(1, -1), (1, 1), (-1, -1), (-1, 1)],
    // long
    lance_black: [(-1, 0), (-2, 0), (-3, 0), (-4, 0), (-5, 0), (-6, 0), (-7, 0), (-8, 0)],
    bishop_black: [
        [(-1, -1), (-2, -2), (-3, -3), (-4, -4), (-5, -5), (-6, -6), (-7, -7), (-8, -8)],
        [(-1, 1), (-2, 2), (-3, 3), (-4, 4), (-5, 5), (-6, 6), (-7, 7), (-8, 8)],
        [(1, -1), (2, -2), (3, -3), (4, -4), (5, -5), (6, -6), (7, -7), (8, -8)],
        [(1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6), (7, 7), (8, 8)],
    ],
    rook_black: [
        [(1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (8, 0)],
        [(-1, 0), (-2, 0), (-3, 0), (-4, 0), (-5, 0), (-6, 0), (-7, 0), (-8, 0)],
        [(0, -1), (0, -2), (0, -3), (0, -4), (0, -5), (0, -6), (0, -7), (0, -8)],
        [(0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7), (0, 8)],
    ],
    // normal
    pawn_white: [(1, 0)],
    knight_white: [(2, -1), (2, 1)],
    silver_white: [(1, -1), (1, 0), (1, 1), (-1, -1), (-1, 1)],
    gold_white: [(1, -1), (1, 0), (1, 1), (0, -1), (0, 1), (-1, 0)],
    king_white: [(1, -1), (1, 0), (1, 1), (0, -1), (0, 1), (-1, -1), (-1, 0), (-1, 1)],
    normal_horse_white: [(1, 0), (0, -1), (0, 1), (-1, 0)],
    normal_dragon_white: [(1, -1), (1, 1), (-1, -1), (-1, 1)],
    // long
    lance_white: [(1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (8, 0)],
    bishop_white: [
        [(-1, -1), (-2, -2), (-3, -3), (-4, -4), (-5, -5), (-6, -6), (-7, -7), (-8, -8)],
        [(-1, 1), (-2, 2), (-3, 3), (-4, 4), (-5, 5), (-6, 6), (-7, 7), (-8, 8)],
        [(1, -1), (2, -2), (3, -3), (4, -4), (5, -5), (6, -6), (7, -7), (8, -8)],
        [(1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6), (7, 7), (8, 8)],
    ],
    rook_white: [
        [(1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (8, 0)],
        [(-1, 0), (-2, 0), (-3, 0), (-4, 0), (-5, 0), (-6, 0), (-7, 0), (-8, 0)],
        [(0, -1), (0, -2), (0, -3), (0, -4), (0, -5), (0, -6), (0, -7), (0, -8)],
        [(0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7), (0, 8)],
    ],
};

