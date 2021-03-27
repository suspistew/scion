use scion::{
    config::{scion_config::ScionConfigBuilder, window_config::WindowConfigBuilder},
    core::{
        components::{
            material::Material2D,
            maths::{
                camera::Camera2D,
                transform::{Coordinates, Transform2D},
            },
            Square,
        },
        game_layer::{GameLayer, SimpleGameLayer},
        resources::inputs::Inputs,
    },
    legion::{system, Resources, World},
    utils::file::app_base_path,
    Scion,
};

#[derive(Debug)]
struct Case(Coordinates);

enum MoveDirection {
    Left,
    Top,
    Right,
    Bottom,
    None,
}

struct Taquin {
    board: [[bool; 4]; 4],
}

impl Taquin {
    fn new() -> Self {
        Self {
            board: [
                [true, true, true, true],
                [true, true, true, true],
                [true, true, true, true],
                [true, true, true, false],
            ],
        }
    }

    fn try_move(&mut self, x: usize, y: usize) -> MoveDirection {
        self.board[x][y] = false;
        if x > 0 && !self.board[x - 1][y] {
            self.board[x - 1][y] = true;
            MoveDirection::Left
        } else if y > 0 && !self.board[x][y - 1] {
            self.board[x][y - 1] = true;
            MoveDirection::Top
        } else if x < 3 && !self.board[x + 1][y] {
            self.board[x + 1][y] = true;
            MoveDirection::Right
        } else if y < 3 && !self.board[x][y + 1] {
            self.board[x][y + 1] = true;
            MoveDirection::Bottom
        } else {
            self.board[x][y] = true;
            MoveDirection::None
        }
    }
}

fn square(x: usize, y: usize) -> Square {
    let x_offset = x as f32 * 0.25;
    let y_offset = y as f32 * 0.25;
    Square::new(
        Coordinates::new(0., 0.),
        192.,
        Some([
            Coordinates::new(x_offset, y_offset),
            Coordinates::new(x_offset, 0.25 + y_offset),
            Coordinates::new(0.25 + x_offset, 0.25 + y_offset),
            Coordinates::new(0.25 + x_offset, y_offset),
        ]),
    )
}

#[system(for_each)]
fn taquin(
    #[resource] inputs: &Inputs,
    #[resource] taquin: &mut Taquin,
    case: &mut Case,
    transform: &mut Transform2D,
) {
    if inputs.mouse().click_event() {
        let mouse_x = inputs.mouse().x();
        let mouse_y = inputs.mouse().y();
        if mouse_x > (case.0.x() * 192.) as f64
            && mouse_y > (case.0.y() * 192.) as f64
            && mouse_x < (case.0.x() * 192. + 192.) as f64
            && mouse_y < (case.0.y() * 192. + 192.) as f64
        {
            match taquin.try_move(case.0.x() as usize, case.0.y() as usize) {
                MoveDirection::Left => {
                    case.0.set_x(case.0.x() - 1.);
                    transform.append_translation(-192., 0.);
                }
                MoveDirection::Top => {
                    case.0.set_y(case.0.y() - 1.);
                    transform.append_translation(0., -192.);
                }
                MoveDirection::Right => {
                    case.0.set_x(case.0.x() + 1.);
                    transform.append_translation(192., 0.);
                }
                MoveDirection::Bottom => {
                    case.0.set_y(case.0.y() + 1.);
                    transform.append_translation(0., 192.);
                }
                MoveDirection::None => {}
            };
        }
    }
}

#[derive(Default)]
struct Layer;

impl SimpleGameLayer for Layer {
    fn on_start(&mut self, world: &mut World, resource: &mut Resources) {
        let p = app_base_path().expect("A base path is mandatory");
        let p = p.join("assets/taquin.png");
        for x in 0..4 {
            for y in 0..4 {
                if !(x == 3 && y == 3) {
                    let square = (
                        Case(Coordinates::new(x as f32, y as f32)),
                        square(x, y),
                        Material2D::Texture(p.as_path().to_str().unwrap().to_string()),
                        Transform2D::new(
                            Coordinates::new(x as f32 * 192., y as f32 * 192.),
                            1.,
                            0.,
                        ),
                    );
                    world.push(square);
                }
            }
        }
        resource.insert(Camera2D::new(768., 768., 10.));
        resource.insert(Taquin::new());
    }
}

fn main() {
    Scion::app_with_config(
        ScionConfigBuilder::new()
            .with_window_config(WindowConfigBuilder::new().with_dimensions((768, 768)).get())
            .get(),
    )
    .with_system(taquin_system())
    .with_game_layer(GameLayer::weak::<Layer>())
    .run();
}
