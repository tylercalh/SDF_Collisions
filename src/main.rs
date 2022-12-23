use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut scene: Vec<&Rectangle> = Vec::new();

    let left = Rectangle::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, screen_height() * 2.0));
    let right = Rectangle::new(Vec2::new(screen_width() - 1.0, 0.0), Vec2::new(1.0, screen_height() * 2.0));
    let top = Rectangle::new(Vec2::new(1.0, 0.0), Vec2::new((screen_width() - 1.0) * 2.0, 1.0));
    let bottom = Rectangle::new(Vec2::new(1.0, screen_height() - 1.0), Vec2::new((screen_width() - 1.0) * 2.0, 1.0));
    //let origin = Rectangle::new(Vec2::new(130.0, 130.0), Vec2::new(50.0, 50.0));


    //let mut ball = Ball::new(Vec2::new(screen_width() / 2.0, screen_height() / 2.0), Vec2::new(-1.0, -0.50));
    let initial_direction = Vec2::new(screen_width(), screen_height()).normalize();
    let mut ball = Ball::new(Vec2::new(screen_width() / 2.0, screen_height() / 2.0), initial_direction);
    let mut can_collide = true;

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

        let scene_info = distance_to_scene(&scene, Vec2::new(ball.position.x, ball.position.y));
        let closest_distance = scene_info.0;
        let closest_rectangle = scene_info.1;

        ball.position = ball.position + ball.direction * get_frame_time() * 300.0;
        //ball.direction.y = ball.direction.y + 980.0 * get_frame_time();
        //ball.position = Vec2::new(mouse_position_x, mouse_position_y);

        if closest_distance >= 15.0 {
             can_collide = true;
        }
        if closest_distance < 15.0 && can_collide {
            can_collide = false;
            ball.draw_ball(RED);
            let normal = calculate_collision_normal(
                Vec2::new(ball.position.x, ball.position.y),
                closest_rectangle.size,
                closest_rectangle.position
            );

            let collision_trajectory = calculate_collision_trajectory(ball.direction, normal);
            //println!("trajectory before collision {} {}", ball.direction.x, ball.direction.y);
            //println!("normal {} {}", normal.x, normal.y);
            //println!("trajectory after collision {} {}", collision_trajectory.x, collision_trajectory.y);
            println!("distance to scene: {}", closest_distance);
            ball.direction = collision_trajectory;

            ball.position.x = ball.position.x + 2.0 * ball.direction.x;
            ball.position.y = ball.position.y + 2.0 * ball.direction.y;

        } else {
            ball.draw_ball(GREEN);
        }

        next_frame().await
    }
}


fn sdf2(point: Vec2, size: Vec2, position: Vec2) -> f32 {
    let point = point - Vec2::new(position.x, position.y);
    let size = size / 2.0;
    let q = Vec2::abs(point) - size;
    let d = Vec2::length(Vec2::max(q, Vec2::ZERO) + f32::min(q.max_element(), 0.0));
    d
}
// p, the position of the collision point
// r, [rectangle width, rectangle height]
// position, the position of the rectangle
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

fn distance_to_scene(scene: &Vec<&Rectangle>, point: Vec2) -> (f32, Rectangle) {
    let mut distance = f32::INFINITY;
    let mut closest_rectangle = scene[0];
    for rectangle in scene {
        //let distance_to_rectangle = sdf(point, rectangle.size, rectangle.position);
        let distance_to_rectangle = sdf2(point, rectangle.size, rectangle.position);
        if distance_to_rectangle < distance {
            distance = distance_to_rectangle;
            closest_rectangle = &rectangle;
        }
    }

    (distance, Rectangle::new(closest_rectangle.position, closest_rectangle.size))
}

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