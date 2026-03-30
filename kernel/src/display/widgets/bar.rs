use core::iter::empty;

use heapless::Vec;

use crate::display::widgets::*;

pub struct Bar<const N: usize> {
    points: Vec<Label, N>,
    full_colour: (u8, u8, u8),
    empty_colour: (u8, u8, u8)
}

impl<const N: usize> Bar<N> {
    pub fn new(
        full_colour: (u8, u8, u8),
        empty_colour: (u8, u8, u8)
    ) -> Self {
        Self {
            points: Vec::new(),
            full_colour,
            empty_colour
        }
    }

    pub fn render_points(
        &mut self,
        parent: *mut _lv_obj_t, // LVGL parent container
        gradient: i8, // Curve of bar in px coordinates
        x: u16,
        y: u16,
        point_char: &'static str, // Character to use for each point
        point_font: *const lv_font_t, // Font to use for each point
        point_space: i8 // Space between points
    ) {
        for i in 0 .. N {
            let new_x = x as i16 + (i as i16 * gradient as i16);
            let new_y = unsafe { y as i16 + (i * ((*point_font).line_height / 2) as usize + i * point_space as usize) as i16 };

            let tmp_label = label(parent, point_char, new_x, new_y);
            tmp_label.set_font(point_font);
            tmp_label.set_colour(self.empty_colour.0, self.empty_colour.1, self.empty_colour.2);
            self.points.push(tmp_label).unwrap();
        }

        self.points.reverse();
    }

    pub fn set_value(&mut self, value: u8) {
        let num_points = self.points.len() as u8;
        let mut colour = self.full_colour;

        // Trigger red if we are at the maximum count
        if value >= num_points {
            colour = (255, 0, 0); 
        }

        for (i, point) in self.points.iter().enumerate() {
            let is_lit = (i as u8 + 1) <= value;

            let c = if is_lit {
                colour
            } else {
                self.empty_colour
            };
            
            point.set_colour(c.0, c.1, c.2);
        }
    }
}

pub fn bar<const N: usize>(
    parent: *mut _lv_obj_t,
    gradient: i8,
    x: u16,
    y: u16,
    point_char: &'static str,
    point_font: *const lv_font_t,
    point_space: i8,
    full_colour: (u8, u8, u8),
    empty_colour: (u8, u8, u8)
) -> Bar<N> {
    let mut b = Bar::new(full_colour, empty_colour);
    b.render_points(parent, gradient, x, y, point_char, point_font, point_space);
    b.set_value(0);
    b
}