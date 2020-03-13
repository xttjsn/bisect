use std::fmt::Debug;
use std::marker::PhantomData;
use log::trace;

pub trait State: PartialEq + Clone {
    type Error;
    fn bisect(&self, end: &Self) -> Result<Self, Self::Error>;
}

pub trait Issue<S: State> {
    fn exists(state: &S) -> bool;
}

#[derive(Debug)]
pub enum BisectorError<S: State> {
    BothEndHaveIssue,
    NoEndHasIssue,
    StateStepError(S::Error),
}

pub struct IssueBisector<I, S>
where
    S: State,
    I: Issue<S>,
{
    state: S,
    end: S,
	nstep: u32,
    phantom: PhantomData<I>,
}

impl<I, S> IssueBisector<I, S>
where
    S: State + Debug,
    I: Issue<S>,
{
    pub fn new(state: S, end: S) -> Self {
        IssueBisector {
            state,
            end,
			nstep: 0,
            phantom: PhantomData,
        }
    }

    pub fn run(&mut self) -> Result<(u32, S), BisectorError<S>> {
        match (I::exists(&self.state), I::exists(&self.end)) {
            (true, true) => Err(BisectorError::BothEndHaveIssue),
            (false, false) => Err(BisectorError::NoEndHasIssue),
            (false, true) => Ok((0, self.state.clone())),
            (true, false) => {
				let (mut l, mut r) = (self.state.clone(), self.end.clone());
				loop {
					self.nstep += 1;
					trace!("nstep incremet to {}", self.nstep);
					let n = match l.bisect(&r) {
						Ok(n) => n,
						Err(e) => return Err(BisectorError::StateStepError(e))
					};
					trace!("new state : {:?}", n);
					if I::exists(&n) {
						trace!("issue exists in new state");
						l = n;
						trace!("l <- n");
					} else {
						trace!("issue does not exist in new state");
						if n == r {
							trace!("found target state");
							return Ok((self.nstep, n));
						} else {
							trace!("r <- n");
							r = n;
						}
					}
				}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Issue, IssueBisector, State};

    #[test]
    fn test_integer_finder() {
        #[derive(Debug, Clone, PartialEq)]
        struct Int {
            pub i: u32,
        }
        impl State for Int {
            type Error = String;
            fn bisect(&self, end: &Self) -> Result<Self, Self::Error> {
                match self.i.checked_add(end.i) {
                    Some(s) => {
                        let dist = if self.i > end.i {
                            match self.i.checked_sub(end.i) {
                                Some(d) => Ok(d),
                                None => Err("Integer Overflow".to_owned()),
                            }
                        } else {
                            match end.i.checked_sub(self.i) {
                                Some(d) => Ok(d),
                                None => Err("Integer Overflow".to_owned()),
                            }
                        }?;

                        Ok(if dist == 1 {
                            Int {
								i: end.i
							}
                        } else {
							Int {
								i: s / 2
							}
                        })
                    }
                    None => Err("Integer Overflow".to_owned()),
                }
            }
        }

        struct WantIntGreaterThan10000Issue {}

        impl Issue<Int> for WantIntGreaterThan10000Issue {
            fn exists(state: &Int) -> bool {
                state.i <= 10000
            }
        }

		use simplelog::*;
		use std::fs::File;

		CombinedLogger::init(vec![
			WriteLogger::new(LevelFilter::Trace, Config::default(), File::create("test_integer_finder.log").unwrap())
		]).unwrap();

        let mut bisector = IssueBisector::<WantIntGreaterThan10000Issue, Int>::new(
            Int { i: 0 },
            Int { i: 100000 },
        );

        let (nstep, finalint) = bisector.run().unwrap();

		assert_eq!(nstep, 18);
        assert_eq!(finalint.i, 10001);
    }
}

//////////////////////////////////// Real Implementations ////////////////////////////
fn main() {

}
