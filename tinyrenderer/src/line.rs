use crate::point::Point;

#[derive(Copy, Clone, Debug)]
pub(crate) struct MajorMinor<T> {
    major: T,
    minor: T,
}

impl<T> MajorMinor<T> {
    fn new(major: T, minor: T) -> Self {
        MajorMinor { major, minor }
    }
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct SlopeParameters {
    is_steep: bool,
    error_threshold: i32,
    derror_step: MajorMinor<i32>,
    position_step: MajorMinor<Point>,
    length: u32,
}

impl SlopeParameters {
    pub(crate) const fn new(
        is_steep: bool,
        error_threshold: i32,
        derror_step: MajorMinor<i32>,
        position_step: MajorMinor<Point>,
        length: u32,
    ) -> Self {
        SlopeParameters {
            is_steep,
            error_threshold,
            derror_step,
            position_step,
            length,
        }
    }
}

impl From<&Line> for SlopeParameters {
    fn from(line: &Line) -> Self {
        let delta = line.end - line.start;
        let dx = delta.x;
        let dy = delta.y;
        let dx_abs = dx.abs();
        let dy_abs = dy.abs();

        let (delta, step, is_steep, length) = if dx_abs < dy_abs {
            (
                MajorMinor::new(dy_abs, dx_abs),
                MajorMinor::new(Point::new(0, dy.signum()), Point::new(dx.signum(), 0)),
                true,
                dy_abs as u32,
            )
        } else {
            (
                MajorMinor::new(dx_abs, dy_abs),
                MajorMinor::new(Point::new(dx.signum(), 0), Point::new(0, dy.signum())),
                false,
                dx_abs as u32,
            )
        };

        SlopeParameters::new(
            is_steep,
            delta.major,
            MajorMinor::new(2 * delta.minor, 2 * delta.major),
            step,
            length,
        )
    }
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct BresenhamWrapper {
    current_point: Point,
    error: i32,
}

impl BresenhamWrapper {
    const fn new(start_point: Point) -> Self {
        Self {
            current_point: start_point,
            error: 0,
        }
    }

    fn next(&mut self, parameters: &SlopeParameters) -> Point {
        if self.error > parameters.error_threshold {
            self.current_point += parameters.position_step.minor;
            self.error -= parameters.derror_step.minor;
        }

        let ret = self.current_point;

        self.current_point += parameters.position_step.major;
        self.error += parameters.derror_step.major;

        ret
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Points {
    parameters: SlopeParameters,
    bresenham: BresenhamWrapper,
    points_remaining: u32,
}

impl Points {
    pub(crate) fn new(line: &Line) -> Self {
        let parameters = SlopeParameters::from(line);
        let bresenham = BresenhamWrapper::new(line.start);
        let points_remaining = parameters.length + 1;

        Self {
            parameters,
            bresenham,
            points_remaining,
        }
    }
}

impl Iterator for Points {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.points_remaining > 0 {
            self.points_remaining -= 1;

            Some(self.bresenham.next(&self.parameters))
        } else {
            None
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub(crate) struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    pub const fn new(start: Point, end: Point) -> Self {
        Line { start, end }
    }

    pub fn points(&self) -> Points {
        Points::new(self)
    }
}
