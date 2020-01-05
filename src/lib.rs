pub struct StarRange<T> {
    start: T,
    end: T,
    step: T,
}

impl Iterator for StarRange<isize> {
    type Item = isize;
    fn next(&mut self) -> Option<Self::Item> {
        let res = self.start;

        if self.step < 0 {
            if res <= self.end {
                None
            } else {
                self.start += self.step;
                Some(res)
            }
        } else {
            if res >= self.end {
                None
            } else {
                self.start += self.step;
                Some(res)
            }
        }
    }
}
impl Iterator for StarRange<f64> {
    type Item = f64;
    fn next(&mut self) -> Option<Self::Item> {
        let res = self.start;

        if self.step < 0. {
            if res <= self.end {
                None
            } else {
                self.start += self.step;
                Some(res)
            }
        } else {
            if res >= self.end {
                None
            } else {
                self.start += self.step;
                Some(res)
            }
        }
    }
}

pub fn range<T>(start: T, end: T, step: T) -> StarRange<T> {
    StarRange {
        start: start,
        end: end,
        step: step,
    }
}
