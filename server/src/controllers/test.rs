pub struct Controllers(pub i32, pub i32);

impl Controllers {
    pub fn add(self) -> i32 {
        let a = self.0 + self.1;
        a
    }
}
