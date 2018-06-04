use cgmath::*;

pub struct Camera
{
    pos: Point3<f32>,
    dir: Vector3<f32>,
    lastx: f32,
    lasty: f32,
    yaw: f32,
    pitch: f32,
}
impl Camera
{
    pub fn new(pos: Point3<f32>, dir: Vector3<f32>) -> Camera
    {
        Camera
        {
            pos: pos,
            dir: dir,
            lastx: 0.0,
            lasty: 0.0,
            yaw: -90.0,
            pitch: 0.0,
        }
    }
    pub fn get_view(&self) -> Matrix4<f32>
    {
        Matrix4::look_at(self.pos,self.pos+self.dir,Vector3::new(0.0,1.0,0.0))
    }
    pub fn move_to(&mut self,key: u32)
    {
        let spd = 0.15;
        match key
        {
            0 =>self.pos+=self.dir*spd,
            1 =>self.pos-=self.dir*spd,
            2 =>self.pos-=self.dir.cross(Vector3::new(0.0,1.0,0.0)).normalize()*spd,
            3 =>self.pos+=self.dir.cross(Vector3::new(0.0,1.0,0.0)).normalize()*spd,
            _ => ()
        }

    }
    pub fn rotate(&mut self,xpos: f32,ypos: f32)
    {
        let sensitivity = 0.1;
        let xoffset = (xpos - self.lastx)*sensitivity;
        let yoffset = (-ypos + self.lasty)*sensitivity;
        self.lastx=xpos;
        self.lasty=ypos;

        self.yaw += xoffset;
        self.pitch += yoffset;

        if self.pitch > 89.0
        {
            self.pitch = 89.0;
        }
        if self.pitch < -89.0
        {
            self.pitch = -89.0;
        }
        self.dir = Vector3::new(
            self.yaw.to_radians().cos()*self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
            ).normalize();

    }
}
