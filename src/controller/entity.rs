struct Pos {
    x: u64,
    y: u64,
}

pub struct EntityBase {
    pos: Pos,
    represent: char,
    desc: Option<String>,
}

impl EntityBase {
    pub fn new(x:u64, y:u64, represent: char, desc: Option<String>) -> Self {
        let gen_pos = Pos {x, y};
        EntityBase { pos: gen_pos, represent, desc: desc }
    }

    pub fn to_coord_item (&self) -> super::CoordItem {
        super::CoordItem { x: self.pos.x as usize,
                           y: self.pos.y as usize,
                           c: self.represent}
    }

    pub fn is_at(&self, x: u64, y: u64) -> bool {
        let out: bool;
        if self.pos.x == x && self.pos.y == y {
            out = true;
        } else {
            out = false;
        }
        out
    }

    pub fn yield_desc(&self) -> Option<String> {
        self.desc.clone()
    }

    pub fn mv (&mut self, x: i64, y: i64) {
        let new_x: i64 = self.pos.x as i64 + x;
        let new_y: i64 = self.pos.y as i64 + y;

        if new_x >= 0 {
            self.pos.x = new_x as u64;
        }
        if new_y >= 0 {
            self.pos.y = new_y as u64;
        }
    }

}
