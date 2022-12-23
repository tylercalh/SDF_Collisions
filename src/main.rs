use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut scene: Vec<&Rectangle> = Vec::new();

    let left = Rectangle::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, screen_height() * 2.0));
    let right = Rectangle::new(Vec2::new(screen_width() - 1.0, 0.0), Vec2::new(1.0, screen_height() * 2.0));
    let top = Rectangle::new(Vec2::new(1.0, 0.0), Vec2::new((screen_width() - 1.0) * 2.0, 1.0));
    let bottom = Rectangle::new(Vec2::new(1.0, screen_height() - 1.0), Vec2::new((screen_width() - 1.0) * 2.0, 1.0));
    //let origin = Rectangle::new(Vec2::new(130.0, 130.0), Vec2::new(50.0, 50.0));

    let initial_direction = Vec2::new(screen_width(), screen_height()).normalize();
    let mut ball = Ball::new(Vec2::new(screen_width() / 2.0, screen_height() / 2.0), initial_direction);

    scene.push(&left);
    scene.push(&right);
    scene.push(&top);
    scene.push(&bottom);
    //scene.push(&origin);

    loop {
        clear_background(BLACK);

        left.draw(RED);
        right.draw(BLUE);
        top.draw(GREEN);
        bottom.draw(YELLOW);
        //origin.draw(WHITE);

        let possible_collisions = detect_collisions(&scene, Vec2::new(ball.position.x, ball.position.y));
        match possible_collisions {
            Some(rectangles) => {
                ball.draw_ball(RED);
                for rectangle in rectangles {
                    let normal = calculate_collision_normal(
                        Vec2::new(ball.position.x, ball.position.y),
                        rectangle.size,
                        rectangle.position
                    );
                    let collision_trajectory = calculate_collision_trajectory(ball.direction, normal);
                    ball.direction = collision_trajectory;
                    println!("trajectory: {}", collision_trajectory);

                }
            },
            None => ball.draw_ball(GREEN),
        };

        ball.position = ball.position + ball.direction * get_frame_time() * 300.0;

        next_frame().await
    }
}


fn sdf(point: Vec2, size: Vec2, position: Vec2) -> f32 {
    let point = point - Vec2::new(position.x, position.y);
    let size = size / 2.0;
    let q = Vec2::abs(point) - size;
    let d = Vec2::length(Vec2::max(q, Vec2::ZERO) + f32::min(q.max_element(), 0.0));
    d
}

fn calculate_collision_normal(p: Vec2, r: Vec2, position: Vec2) -> Vec2 {
    let p = p - position;
    
    let r = r / 2.0;
    let d = Vec2::abs(p) - r;
    let normal = d.max(Vec2::ZERO);
    normal.normalize()
}

fn calculate_collision_trajectory(direction: Vec2, normal: Vec2) -> Vec2 {
    let direction = 2.0 * (direction * normal * Vec2::NEG_ONE) + direction;
    direction.normalize()
}

fn detect_collisions(scene: &Vec<&Rectangle>, point: Vec2) -> Option<Vec<Rectangle>> {
    let mut rectangles: Vec<Rectangle> = Vec::new();

    for rectangle in scene {
        let distance_to_rectangle = sdf(point, rectangle.size, rectangle.position);
        let rec = rectangle.clone();
        if distance_to_rectangle < 15.0 {
            rectangles.push(*rec);
        }
    }
    if rectangles.len() > 0 {
        Some(rectangles)
    } else {
        None
    }
}

#[derive(Copy, Clone)]
struct Rectangle {
    position: Vec2,
    size: Vec2,
}

impl Rectangle {
    fn new(position: Vec2, size: Vec2) -> Rectangle {
        Rectangle {
            position: position,
            size: size,
        }
    }

    fn draw(&self, color: Color) {
        draw_rectangle(
            self.position.x - self.size.x / 2.0,
            self.position.y - self.size.y / 2.0,
            self.size.x, 
            self.size.y, color);
    }
}

struct Ball {
    position: Vec2,
    direction: Vec2,
}

impl Ball {
    fn new(position: Vec2, direction: Vec2) -> Ball {
        Ball {
            position: position,
            direction: direction,
        }
    }

    fn draw_ball(&self, color: Color) {
        draw_circle(self.position.x, self.position.y, 15.0, color);
        draw_line(
            self.position.x,
            self.position.y,
            self.position.x + self.direction.x * 50.0,
            self.position.y + self.direction.y * 50.0,
            3.0,
            RED
        );
    }
}