fn main() {}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Position2d {
    x: f64,
    y: f64,
}

impl Position2d {
    fn add(self, v: Vector2d) -> Position2d {
        return Position2d {
            x: self.x + v.x,
            y: self.y + v.y,
        };
    }
}

fn pos2(x: f64, y: f64) -> Position2d {
    return Position2d { x: x, y: y };
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Vector2d {
    x: f64,
    y: f64,
}

fn vec2(x: f64, y: f64) -> Vector2d {
    return Vector2d { x: x, y: y };
}

impl Vector2d {
    fn add(self, v: Vector2d) -> Vector2d {
        return Vector2d {
            x: self.x + v.x,
            y: self.y + v.y,
        };
    }

    fn times(self, scalar: f64) -> Vector2d {
        return Vector2d {
            x: self.x * scalar,
            y: self.y * scalar,
        };
    }
}

#[derive(Debug, Copy, Clone)]
struct Projectile {
    position: Position2d,
    velocity: Vector2d,
    acceleration: Vector2d,
}

impl Projectile {
    fn update(self, time_delta_seconds: f64) -> Projectile {
        let new_pos = self.position.add(self.velocity.times(time_delta_seconds));
        let new_vel = self.velocity
            .add(self.acceleration.times(time_delta_seconds));
        println!("projectile updated to {:?}, {:?}", new_pos, new_vel);
        return Projectile {
            position: new_pos,
            velocity: new_vel,
            acceleration: self.acceleration.clone(),
        };
    }
}

#[derive(Debug, Copy, Clone)]
struct Tank {
    health: i8,
    barrel_angle: i8,
    barrel_length: f64,
    position: Position2d,
}

impl Tank {
    fn is_alive(self) -> bool {
        return self.health > 0;
    }

    fn shoot(self, power: i8) -> Projectile {
        // TODO: shouldn't be here
        let gravity = -10.0;

        return Projectile {
            position: self.position
                .add(self.barrel_vector().times(self.barrel_length)),
            acceleration: vec2(0.0, gravity),
            velocity: self.barrel_vector().times(power.into()),
        };
    }

    fn hit(self, projectile: Projectile) -> Tank {
        println!("i'm hit");
        return Tank {
            health: self.health - 50,
            barrel_angle: self.barrel_angle,
            barrel_length: self.barrel_length,
            position: self.position,
        };
    }

    fn barrel_vector(self) -> Vector2d {
        let angle: f64 = self.barrel_angle.into();
        return vec2(angle.to_radians().cos(), angle.to_radians().sin());
    }
}

#[derive(Debug, Clone)]
struct World {
    wind: i8,
    tanks: Vec<Tank>,
    projectiles: Vec<Projectile>,
}

impl World {
    fn update(self, time_delta_seconds: f64) -> World {
        let p2s = &self.projectiles.clone();

        let t2: Vec<Tank> = self.tanks
            .iter()
            .map(|t| {
                let mut tp = t.clone();

                // TODO: move hit detection out
                // TODO: randomize projectile iteration
                for p2 in p2s {
                    let distance = ((tp.position.x - p2.position.x).powi(2)
                        + (tp.position.y - p2.position.y).powi(2))
                        .sqrt();

                    println!("distance {}", distance);

                    if distance < 1.0 {
                        // TODO: kill projectiles after a hit
                        tp = tp.hit(*p2)
                    }
                }

                // TODO: remove dead tanks

                return tp;
            })
            .collect();
        return World {
            wind: self.wind,
            tanks: t2,
            projectiles: self.projectiles
                .iter() // TODO: remove dead projectiles
                .map(|p| p.update(time_delta_seconds))
                .collect(),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn updates_position() {
        let m = Projectile {
            position: pos2(0.0, 100.0),
            velocity: vec2(100.0, 10.0),
            acceleration: vec2(-10.0, -10.0),
        };

        let m2 = m.update(1.0);

        assert_eq!(m2.position, pos2(100.0, 110.0));
        assert_eq!(m2.velocity, vec2(90.0, 0.0));
        assert_eq!(m2.acceleration, vec2(-10.0, -10.0));
    }

    #[test]
    fn dead_tank() {
        let t = Tank {
            health: 0,
            barrel_angle: 90,
            barrel_length: 0.5,
            position: pos2(0.0, 0.0),
        };
        assert!(t.is_alive() == false);
    }

    #[test]
    fn alive_tank() {
        let t = Tank {
            health: 1,
            barrel_angle: 90,
            barrel_length: 0.5,
            position: pos2(0.0, 0.0),
        };
        assert!(t.is_alive() == true);
    }

    #[test]
    fn shoot() {
        let t = Tank {
            health: 1,
            barrel_angle: 90,
            barrel_length: 0.5,
            position: pos2(0.0, 0.0),
        };
        let power = 100;
        let projectile = t.shoot(power);
        assert!(projectile.position.x.to_string().starts_with("0.0"));
        assert!(projectile.position.y.to_string().starts_with("0.5"));
        assert!(projectile.velocity.x.to_string().starts_with("0.0"));
        assert!(projectile.velocity.y.to_string().starts_with("100"));
        assert_eq!(projectile.acceleration, vec2(0.0, -10.0)); // TODO: move gravity out
    }

    #[test]
    fn hit() {
        let t1 = Tank {
            health: 100,
            barrel_angle: 90,
            barrel_length: 0.5,
            position: pos2(0.0, 0.0),
        };

        let power = 100;

        // damage?
        let projectile = t1.shoot(power);

        let t2 = t1.hit(projectile);

        assert_eq!(t2.health, 50);
    }

    #[test]
    fn barrel_vector() {
        let t1 = Tank {
            health: 100,
            barrel_angle: 45,
            barrel_length: 0.5,
            position: pos2(0.0, 0.0),
        };

        let barrel_unit_vector = t1.barrel_vector();

        assert!(barrel_unit_vector.x.to_string().starts_with("0.707"));
        assert!(barrel_unit_vector.y.to_string().starts_with("0.707"));
    }

    #[test]
    fn shoot_up_get_hit_by_own_bullet() {
        let t1 = Tank {
            health: 1,
            barrel_angle: 90,
            barrel_length: 0.5,
            position: pos2(0.0, 0.0),
        };

        let mut w = World {
            wind: 0,
            tanks: vec![t1],
            projectiles: vec![t1.shoot(20)],
        };

        for _ in 0..200 {
            w = w.update(0.1);
        }

        assert!(!w.tanks[0].is_alive());
    }

}
