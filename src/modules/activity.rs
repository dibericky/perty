#[derive(Debug, PartialEq)]
pub struct Estimation {
    pub optimistic: u32,
    pub probable: u32,
    pub pessimistic: u32,
}

impl Estimation {
    pub fn estimated(&self) -> f64 {
        let tm = f64::from(self.probable);
        let to = f64::from(self.optimistic);
        let tp = f64::from(self.pessimistic);
        1f64 / 6f64 * (4f64 * tm + to + tp)
    }
}

#[derive(Debug, PartialEq)]
pub struct Activity {
    pub name: String,
    pub estimation: Estimation,
}

impl Activity {
    pub fn new(name: String, optimistic: u32, probable: u32, pessimistic: u32) -> Self {
        Self {
            name,
            estimation: Estimation {
                optimistic,
                probable,
                pessimistic,
            },
        }
    }

    pub fn estimated(&self) -> f64 {
        self.estimation.estimated()
    }
}

#[cfg(test)]
mod test {
    use super::Activity;

    #[test]
    fn get_estimated() {
        let estimation = Activity::new("activity 1".to_string(), 6, 10, 15).estimated();
        assert_eq!(estimation, 10.166666666666666);
    }
}
