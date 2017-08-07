use std::env;

use screen::{Screen, Pen};
use speed::Speed;
use point::Point;
use ideoutput::IDEOutput;
use angle::Direction;

pub use angle::Angle;

/// This type represents any distance value
pub type Distance = f64;

/// A turtle with a pen attached to its tail
pub struct Turtle {
    screen: Box<Screen>,
    speed: Speed,
    position: Point,
    direction: Direction,
    pen: Pen,
}

impl Turtle {
    /// Initialize a new Turtle instance
    pub fn new() -> Turtle {
        let screen: Box<Screen> = match env::args().any(|a| a == "--turtle-ide-print-mode") {
            false => unimplemented!(),
            true => Box::new(IDEOutput::new()),
        };
        Turtle {
            // Attempt to automatically detect if this is running within the Turtle IDE
            screen: screen,
            speed: "normal".into(),
            position: Point::origin(),
            direction: Direction::zero_degrees(),
            pen: Pen::default(),
        }
    }

    /// Returns the current speed of the turtle
    pub fn speed(&self) -> Speed {
        self.speed
    }

    /// Return the turtle's current location (x, y)
    pub fn position(&self) -> Point {
        self.position
    }

    /// Return the turtle's current heading
    pub fn heading(&self) -> Angle {
        self.direction.raw_angle()
    }

    /// Set the turtle's speed to the given setting.
    ///
    /// Usually this method is used as shown below:
    ///
    /// ```rust,no_run
    /// # extern crate turtleide;
    /// # fn main() {
    /// # let mut turtle = turtleide::Turtle::new();
    /// turtle.set_speed("normal".into());
    /// turtle.set_speed("fast".into());
    /// turtle.set_speed(2.into());
    /// turtle.set_speed(10.into());
    /// # }
    /// ```
    ///
    /// If input is a number greater than 10 or smaller than 1,
    /// speed is set to 0 (Speed::Instant). Strings are converted as follows:
    ///
    /// * "slowest" => Speed::One
    /// * "slow" => Speed::Three
    /// * "normal" => Speed::Six
    /// * "fast" => Speed::Ten
    /// * "fastest" => Speed::Instant
    /// * anything else will cause the program to `panic!` at runtime
    ///
    /// Speeds from 1 to 10 enforce increasingly faster animation of
    /// line drawing and turtle turning.
    ///
    /// Note: speed = 0 means that no animation takes place. forward/back
    /// makes turtle jump and likewise left/right make the turtle turn instantly.
    ///
    /// Using this type is an excellent way to learn about conversion
    /// traits `From` and `Into`. Rather than instantiating `Speed`
    /// directly using `Speed::Six`, you usually use `6.into()`. This is the
    /// same as using `Speed::from(6)` but much more compact. This works because
    /// any type that implements the `From` trait gets a matching implementation
    /// of the `Into` trait.
    pub fn set_speed(&mut self, speed: Speed) {
        self.speed = speed;
    }

    /// Change the angle unit to degrees.
    pub fn use_degrees(&mut self) {
        self.direction = self.direction.into_degrees();
    }

    /// Change the angle unit to radians.
    pub fn use_radians(&mut self) {
        self.direction = self.direction.into_radians();
    }

    /// Move the turtle forward by the given amount of `distance`.
    ///
    /// `distance` is given in "pixels" which are like really small turtle steps.
    /// `distance` can be negative in which case the turtle can move backward
    /// using this method.
    pub fn forward(&mut self, distance: Distance) {
        let start = self.position;
        self.position = self.position.translate(self.direction, distance);
        self.screen.draw_line(start, self.position, self.speed, self.pen);
    }

    /// Move the turtle backward by the given amount of `distance`.
    ///
    /// `distance` is given in "pixels" which are like really small turtle steps.
    /// `distance` can be negative in which case the turtle can move forwards
    /// using this method.
    pub fn backward(&mut self, distance: Distance) {
        self.forward(-distance);
    }

    /// Rotate the turtle right by the given angle.
    ///
    /// Units are by default degrees, but can be set using the methods
    /// [`Turtle::use_degrees`](struct.Turtle.html#method.use_degrees) or
    /// [`Turtle::use_radians`](struct.Turtle.html#method.use_radians).
    pub fn right(&mut self, angle: Angle) {
        self.direction.rotate_clockwise(angle);
    }

    /// Rotate the turtle left by the given angle.
    ///
    /// Units are by default degrees, but can be set using the methods
    /// [`Turtle::use_degrees`](struct.Turtle.html#method.use_degrees) or
    /// [`Turtle::use_radians`](struct.Turtle.html#method.use_radians).
    pub fn left(&mut self, angle: Angle) {
        self.direction.rotate_counterclockwise(angle);
    }
}