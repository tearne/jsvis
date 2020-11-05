use rand::rngs::ThreadRng;
use rand::Rng;
use rgb::RGB8;

pub struct Pixel {
    position: f64,
    step_size: f64,
    going_up: bool,
    target_colour: RGB8,
    current_colour: RGB8,
    adjust_step: u8,
}
    
impl Pixel {
    pub fn new(rng: &mut ThreadRng) -> Self {
        Pixel{
            position: rng.gen(),
            step_size: rng.gen::<f64>() * 0.03,
            going_up: rng.gen_bool(0.5), 
            target_colour: RGB8::new(rng.gen(), rng.gen(),rng.gen()),
            current_colour: RGB8::new(0,0,0),
            adjust_step: 1,
        }
    }
    
    pub fn evolve_and_get(&mut self) -> RGB8 {
        if self.going_up {
            self.position += self.step_size
        } else {
            self.position -= self.step_size
        }

        if self.position > 1.0 {
            self.position = 1.0;
            self.going_up = false;
        } else if self.position < 0.05 {
            self.position = 0.05;
            self.going_up = true;
        }

        fn step_towards(actual: &mut u8, target: &u8, adjust_step: u8) {
            if target != actual {
                let diff = *target as f64 - *actual as f64;
                let direction = diff / diff.abs();
                *actual = (direction * (adjust_step as f64) + (*actual as f64)) as u8;
            }
        }

        step_towards(&mut self.current_colour.r, &self.target_colour.r, self.adjust_step);
        step_towards(&mut self.current_colour.g, &self.target_colour.g, self.adjust_step);
        step_towards(&mut self.current_colour.b, &self.target_colour.b, self.adjust_step);

        rgb::RGB8::new(
            (self.current_colour.r as f64 * self.position) as u8,
            (self.current_colour.g as f64 * self.position) as u8, 
            (self.current_colour.b as f64 * self.position) as u8 
        )
    }

    pub fn randomise(&mut self, rng: &mut ThreadRng, target_colour: RGB8) {
        self.step_size = rng.gen::<f64>() * 0.03;
        self.target_colour = target_colour;
        self.adjust_step = rng.gen_range(1, 10);
    }
}
