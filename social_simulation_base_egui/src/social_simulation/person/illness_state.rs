pub enum IllnessState {
    Healthy,
    Infected,
    Recovered,
    Dead,
}


impl PartialEq for IllnessState {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}