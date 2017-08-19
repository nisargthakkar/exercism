pub struct Roman {

}
// I: 1
// V: 5
// X: 10
// L: 50
// C: 100
// D: 500
// M: 1000
impl Roman{
    pub fn from(mut num: u32) -> String {
        let mut s = String::new();
        while num > 0 {
            if num >= 1000 {
                s.push('M');
                num -= 1000;
            } else if num >= 900 {
                s.push_str("CM");
                num -= 900;
            } else if num >= 500 {
                s.push('D');
                num -= 500;
            } else if num >= 400 {
                s.push_str("CD");
                num -= 400;
            } else if num >= 100 {
                s.push('C');
                num -= 100;
            } else if num >= 90 {
                s.push_str("XC");
                num -= 90;
            } else if num >= 50 {
                s.push('L');
                num -= 50;
            } else if num >= 40 {
                s.push_str("XL");
                num -= 40;
            } else if num >= 10 {
                s.push('X');
                num -= 10;
            } else if num >= 9 {
                s.push_str("IX");
                num -= 9;
            } else if num >= 5 {
                s.push('V');
                num -= 5;
            } else if num >= 4 {
                s.push_str("IV");
                num -= 4;
            } else if num >= 1 {
                s.push('I');
                num -= 1;
            }
        }

        s
    }
}