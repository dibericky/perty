pub type EstimationValue = i32;

#[derive(Debug, PartialEq)]
pub struct Estimation {
    pub optimistic: EstimationValue,
    pub probable: EstimationValue,
    pub pessimistic: EstimationValue,
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
    pub fn new(
        name: String,
        optimistic: EstimationValue,
        probable: EstimationValue,
        pessimistic: EstimationValue,
    ) -> Self {
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
