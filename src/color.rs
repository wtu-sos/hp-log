use std::fmt;
use crate::filter::FilterLevel;
/*
 * \x1b[显示方式;前景色;背景色m输出字符串\x1b[0m
 * 显示：0(默认)、1(粗体/高亮)、22(非粗体)、4(单条下划线)、24(无下划线)、5(闪烁)、25(无闪烁)、7(反显、翻转前景色和背景色)、27(无反显)
 * 颜色：0(黑)、1(红)、2(绿)、 3(黄)、4(蓝)、5(洋红)、6(青)、7(白)
 *      前景色为30+颜色值，如31表示前景色为红色；背景色为40+颜色值，如41表示背景色为红色
 * */

#[allow(dead_code)]
pub struct RichContent<T>
where
T: fmt::Display,
{
    text: T,
    fg: Color,
    bg: Color,
}

impl<T: fmt::Display> RichContent<T> {
    pub fn new(content: T, level: FilterLevel) -> Self {
        RichContent {
            text: content,
            fg: level.fg_color(),
            bg: level.bg_color(),
        }
    }
}

impl<T> fmt::Display for RichContent<T>
where
T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1B[0;3{};4{}m", self.fg.color_byte(), self.bg.color_byte())?;
        fmt::Display::fmt(&self.text, f)?;
        write!(f, "\x1B[0m")?;
        Ok(())
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
} 

impl Color {
    fn color_byte(&self) -> char {
        match *self {
            Color::Black => '0',
            Color::Red => '1',
            Color::Green => '2',
            Color::Yellow => '3',
            Color::Blue => '4',
            Color::Magenta => '5',
            Color::Cyan => '6',
            Color::White => '7',
        }
    }
}

