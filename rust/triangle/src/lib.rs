pub struct Triangle {
    side_a: u32,
    side_b: u32,
    side_c: u32
}

impl Triangle {
    pub fn build(mut sides: [u32; 3]) -> Result<Triangle, &'static str> {
        // sum_of_two_sides_must_equal_or_exceed_the_remaining_side
        // equilateral_triangles_have_equal_sides
        // isosceles_triangles_have_two_equal_sides
        // scalene_triangle_has_no_equal_sides
        
        sides.sort();

        let side_a = sides[0];
        let side_b = sides[1];
        let side_c = sides[2];

        if side_a == 0 {
            return Err("Can not have sides with length 0");
        }

        if side_a + side_b < side_c {
            return Err("Sum of two sides can not be less than the third side");
        }

        Ok(Triangle {
            side_a: side_a,
            side_b: side_b,
            side_c: side_c})
    }

    pub fn is_equilateral(&self) -> bool {
        self.side_a == self.side_c
    }

    pub fn is_scalene(&self) -> bool {
        self.side_a != self.side_b && self.side_b != self.side_c
    }

    pub fn is_isosceles(&self) -> bool {
        self.side_a == self.side_b || self.side_b == self.side_c
    }
}