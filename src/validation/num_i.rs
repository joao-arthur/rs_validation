#[derive(Debug, PartialEq, Clone)]
pub struct NumIValidation {
    pub required: bool,
    pub eq: Option<i64>,
    pub ne: Option<i64>,
    pub gt: Option<i64>,
    pub lt: Option<i64>,
    pub ge: Option<i64>,
    pub le: Option<i64>,
}

impl Default for NumIValidation {
    fn default() -> Self {
        NumIValidation {
            required: false,
            eq: None,
            ne: None,
            gt: None,
            lt: None,
            ge: None,
            le: None,
        }
    }
}

impl NumIValidation {
    pub fn required(self) -> Self {
        NumIValidation {
            required: true,
            eq: self.eq,
            ne: self.ne,
            gt: self.gt,
            lt: self.lt,
            ge: self.ge,
            le: self.le,
        }
    }

    pub fn eq(self, value: i64) -> Self {
        NumIValidation {
            required: self.required,
            eq: Some(value),
            ne: self.ne,
            gt: self.gt,
            lt: self.lt,
            ge: self.ge,
            le: self.le,
        }
    }

    pub fn ne(self, value: i64) -> Self {
        NumIValidation {
            required: self.required,
            eq: self.eq,
            ne: Some(value),
            gt: self.gt,
            lt: self.lt,
            ge: self.ge,
            le: self.le,
        }
    }

    pub fn gt(self, value: i64) -> Self {
        NumIValidation {
            required: self.required,
            eq: self.eq,
            ne: self.ne,
            gt: Some(value),
            lt: self.lt,
            ge: self.ge,
            le: self.le,
        }
    }

    pub fn lt(self, value: i64) -> Self {
        NumIValidation {
            required: self.required,
            eq: self.eq,
            ne: self.ne,
            gt: self.gt,
            lt: Some(value),
            ge: self.ge,
            le: self.le,
        }
    }

    pub fn ge(self, value: i64) -> Self {
        NumIValidation {
            required: self.required,
            eq: self.eq,
            ne: self.ne,
            gt: self.gt,
            lt: self.lt,
            ge: Some(value),
            le: self.le,
        }
    }

    pub fn le(self, value: i64) -> Self {
        NumIValidation {
            required: self.required,
            eq: self.eq,
            ne: self.ne,
            gt: self.gt,
            lt: self.lt,
            ge: self.ge,
            le: Some(value),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_num_i_validation() {
        assert_eq!(
            NumIValidation::default(),
            NumIValidation {
                required: false,
                eq: None,
                ne: None,
                gt: None,
                lt: None,
                ge: None,
                le: None,
            }
        );
        assert_eq!(
            NumIValidation::default().required(),
            NumIValidation {
                required: true,
                eq: None,
                ne: None,
                gt: None,
                lt: None,
                ge: None,
                le: None,
            }
        );
        assert_eq!(
            NumIValidation::default().required().eq(-1),
            NumIValidation {
                required: true,
                eq: Some(-1),
                ne: None,
                gt: None,
                lt: None,
                ge: None,
                le: None,
            }
        );
        assert_eq!(
            NumIValidation::default().required().eq(-1).ne(-2),
            NumIValidation {
                required: true,
                eq: Some(-1),
                ne: Some(-2),
                gt: None,
                lt: None,
                ge: None,
                le: None,
            }
        );
        assert_eq!(
            NumIValidation::default().required().eq(-1).ne(-2).gt(-3),
            NumIValidation {
                required: true,
                eq: Some(-1),
                ne: Some(-2),
                gt: Some(-3),
                lt: None,
                ge: None,
                le: None,
            }
        );
        assert_eq!(
            NumIValidation::default().required().eq(-1).ne(-2).gt(-3).lt(-4),
            NumIValidation {
                required: true,
                eq: Some(-1),
                ne: Some(-2),
                gt: Some(-3),
                lt: Some(-4),
                ge: None,
                le: None,
            }
        );
        assert_eq!(
            NumIValidation::default().required().eq(-1).ne(-2).gt(-3).lt(-4).ge(-5),
            NumIValidation {
                required: true,
                eq: Some(-1),
                ne: Some(-2),
                gt: Some(-3),
                lt: Some(-4),
                ge: Some(-5),
                le: None,
            }
        );
        assert_eq!(
            NumIValidation::default().required().eq(-1).ne(-2).gt(-3).lt(-4).ge(-5).le(-6),
            NumIValidation {
                required: true,
                eq: Some(-1),
                ne: Some(-2),
                gt: Some(-3),
                lt: Some(-4),
                ge: Some(-5),
                le: Some(-6),
            }
        );
    }
}
