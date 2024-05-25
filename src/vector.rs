struct Vec3 {
    x: f64,
    y: f64,
    z: f64
}

impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {x: x, y: y, z: z}
    }
    
    fn len_sqr(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    
    fn len(&self) -> f64 {
        let squared = self.len_sqr();
        
        squared.sqrt()
    }
    
    fn normalized(&self) -> Vec3 {
        let inv_len = 1f64 / self.len();
        
        Vec3 {x: self.x * inv_len, y: self.y * inv_len, z: self.z * inv_len}
    }
}

fn dot(a: Vec3, b: Vec3) -> f64 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

fn cross(a: Vec3, b: Vec3) -> Vec3 {
    
    Vec3 { 
        x: a.y * b.z - b.y * a.z,
        y: a.x * b.z - b.x * a.z,
        z: a.x * b.y - b.x * a.y
    }
}
