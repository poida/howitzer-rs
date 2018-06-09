fn main() {}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Position2d {
    X: f64,
    Y: f64,
}

impl Position2d {
    fn add(self, v: Vector2d) -> Position2d {
        return Position2d {
            X: self.X + v.X,
            Y: self.Y + v.Y,
        };
    }
}

fn pos2(x: f64, y: f64) -> Position2d {
    return Position2d { X: x, Y: y };
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Vector2d {
    X: f64,
    Y: f64,
}

fn vec2(x: f64, y: f64) -> Vector2d {
    return Vector2d { X: x, Y: y };
}

impl Vector2d {
    fn add(self, v: Vector2d) -> Vector2d {
        return Vector2d {
            X: self.X + v.X,
            Y: self.Y + v.Y,
        };
    }

    fn times(self, scalar: f64) -> Vector2d {
        return Vector2d {
            X: self.X * scalar,
            Y: self.Y * scalar,
        };
    }
}

#[derive(Debug, Copy, Clone)]
struct Projectile {
    Position: Position2d,
    Velocity: Vector2d,
    Acceleration: Vector2d,
}

impl Projectile {
    fn update(self, timeDeltaSeconds: f64) -> Projectile {
        return Projectile {
            Position: self.Position.add(self.Velocity.times(timeDeltaSeconds)),
            Velocity: self.Velocity.add(self.Acceleration.times(timeDeltaSeconds)),
            Acceleration: self.Acceleration.clone(),
        };
    }
}

#[derive(Debug, Copy, Clone)]
struct Tank {
    Health: i8,
    BarrelAngle: i8,
    BarrelLength: f64,
    Position: Position2d,
}

impl Tank {
    fn is_alive(self) -> bool {
        return self.Health > 0;
    }

    fn shoot(self, power: i8) -> Projectile {
        return Projectile {
            Position: self.Position
                .add(self.barrel_vector().times(self.BarrelLength)),
            Acceleration: vec2(0.0, 0.0),
            Velocity: self.barrel_vector().times(power.into()),
        };
    }

    fn hit(self, projectile: Projectile) -> Tank {
        return Tank {
            Health: self.Health - 50,
            BarrelAngle: self.BarrelAngle,
            BarrelLength: self.BarrelLength,
            Position: self.Position,
        };
    }

    fn barrel_vector(self) -> Vector2d {
        let angle: f64 = self.BarrelAngle.into();
        return vec2(angle.to_radians().cos(), angle.to_radians().sin());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn updatesPosition() {
        let m = Projectile {
            Position: pos2(0.0, 100.0),
            Velocity: vec2(100.0, 10.0),
            Acceleration: vec2(-10.0, -10.0),
        };

        let m2 = m.update(1.0);

        assert_eq!(m2.Position, pos2(100.0, 110.0));
        assert_eq!(m2.Velocity, vec2(90.0, 0.0));
        assert_eq!(m2.Acceleration, vec2(-10.0, -10.0));
    }

    #[test]
    fn deadTank() {
        let t = Tank {
            Health: 0,
            BarrelAngle: 90,
            BarrelLength: 0.5,
            Position: pos2(0.0, 0.0),
        };
        assert!(t.is_alive() == false);
    }

    #[test]
    fn aliveTank() {
        let t = Tank {
            Health: 1,
            BarrelAngle: 90,
            BarrelLength: 0.5,
            Position: pos2(0.0, 0.0),
        };
        assert!(t.is_alive() == true);
    }

    #[test]
    fn shoot() {
        let t = Tank {
            Health: 1,
            BarrelAngle: 90,
            BarrelLength: 0.5,
            Position: pos2(0.0, 0.0),
        };
        let power = 100;
        let projectile = t.shoot(power);
        assert!(projectile.Position.X.to_string().starts_with("0.0"));
        assert!(projectile.Position.Y.to_string().starts_with("0.5"));
        assert!(projectile.Velocity.X.to_string().starts_with("0.0"));
        assert!(projectile.Velocity.Y.to_string().starts_with("100"));
        assert_eq!(projectile.Acceleration, vec2(0.0, 0.0));
    }

    #[test]
    fn hit() {
        let t1 = Tank {
            Health: 100,
            BarrelAngle: 90,
            BarrelLength: 0.5,
            Position: pos2(0.0, 0.0),
        };

        let power = 100;

        // damage?
        let projectile = t1.shoot(power);

        let t2 = t1.hit(projectile);

        assert_eq!(t2.Health, 50);
    }

    #[test]
    fn barrel_vector() {
        let t1 = Tank {
            Health: 100,
            BarrelAngle: 45,
            BarrelLength: 0.5,
            Position: pos2(0.0, 0.0),
        };

        let barrelUnitVector = t1.barrel_vector();

        assert!(barrelUnitVector.X.to_string().starts_with("0.707"));
        assert!(barrelUnitVector.Y.to_string().starts_with("0.707"));
    }
}
