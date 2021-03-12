// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

#![feature(const_in_array_repeat_expressions)]

#![allow(unused_must_use)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod parameter;
mod snake;
mod skin_manager;

use parameter::*;
use snake::{SimpleDirection, ComplexDirection, Tile};
use skin_manager::*;

use std::str::FromStr;
use seed::{prelude::*, *};



// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    
    orders.stream(streams::document_event(Ev::KeyDown, |e| key_update(e)));
    orders.perform_cmd(cmds::timeout(0, || Msg::Init));
    Model {
        board: snake::Board::new(),
        run: false,
        death: false,
        clock: 150,
        laps: 0,
        timer: snake::Timer::new(),
        apple: 0,
        score: 0,
        speed: Speed::Normal,
        skin_apple: SkinApple::Apple,
        skin_snake: SkinSnake::Snake,
        skin_manager: SkinManager::new(),
    }
}

fn key_update(event: web_sys::Event) -> Msg {
    match browser::dom::cast::to_keyboard_event(&event).key().as_ref() {
        "ArrowUp" => Msg::Arrow(SimpleDirection::Up),
        "ArrowDown" => Msg::Arrow(SimpleDirection::Down),
        "ArrowLeft" => Msg::Arrow(SimpleDirection::Left),
        "ArrowRight" => Msg::Arrow(SimpleDirection::Right),
        " " => Msg::Play,
        _ => Msg::None,
    }
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
struct Model {
    board: snake::Board,
    run: bool,
    death: bool,
    clock: u32,
    laps: u16,
    timer: snake::Timer,
    apple: u16,
    score: u64,
    speed: Speed,
    skin_apple: SkinApple,
    skin_snake: SkinSnake,
    skin_manager: SkinManager,
}

impl Model {
    fn init_game(&mut self) {
        self.board = snake::Board::new();
        self.timer = snake::Timer::new();
        self.apple = 0;
        self.score = 0;
        self.run = true;
        self.death = false;
        self.laps = 0;
    }
}

// ------ ------
//    Update
// ------ ------

// `Msg` describes the different events you can modify state with.
enum Msg {
    Init,
    Play,
    Death,
    Arrow(SimpleDirection),
    UpdateBoard,
    UpdateSpeed(parameter::Speed),
    UpdateSkinSnake(parameter::SkinSnake),
    UpdateSkinApple(parameter::SkinApple),
    None,
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Init => {
            document().get_element_by_id("2").unwrap().set_attribute("checked", "true");
            document().get_element_by_id("apple").unwrap().set_attribute("checked", "true");
            document().get_element_by_id("snake").unwrap().set_attribute("checked", "true");
        }
        Msg::Play => {
            if !model.run {
                model.init_game();
                model.clock = match model.speed {
                    Speed::Slow => 175,
                    Speed::Normal => 125,
                    Speed::Fast => 75
                };
                model.skin_manager.set_current_skin_apple(&model.skin_apple);
                model.skin_manager.set_current_skin_snake(&model.skin_snake);
                orders.perform_cmd(cmds::timeout(0, || Msg::UpdateBoard));
            }
        },
        Msg::Death => {
            model.run = false;
            model.death = true;
        }
        Msg::Arrow(d) => model.board.set_direction(d),
        Msg::UpdateBoard => {
            model.laps += 1;
            if let Some(b) = model.board.grow_snake() {
                if !(model.laps <= 3) {
                    if !b {
                        model.board.move_queue()
                    } else {
                        model.apple += 1;
                        model.score += ((1.0 / (model.timer.time_since_last_eat + 10.) * 3000.
                            + 200.)
                            * (model.apple as f64 / 100.0 + 1.))
                            as u64;
                        model.timer.time_since_last_eat = 0.;
                        model.board.generate_apple()
                    }
                }
                model.board.update_board();
                model.timer.add(model.clock as f64 / 1000.);
                orders.perform_cmd(cmds::timeout(model.clock, || Msg::UpdateBoard));
            } else {
                orders.perform_cmd(cmds::timeout(0, || Msg::Death));
            }
        }
        Msg::UpdateSpeed(x) => {
            if !model.run {
                model.speed = x;
            }
        }
        Msg::UpdateSkinApple(x) => {
                model.skin_apple = x;
        }
        Msg::UpdateSkinSnake(x) => {                        
            model.skin_snake = x;
        }
        Msg::None => (),
    }
}


// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    div![
        C!["page"],
         div![
            C!["center-verticaly"],
            parameter(&model),
            game(&model),
            record(&model)
        ]
    ]
}



fn parameter(model: &Model) -> Node<Msg> {
    div![
        C!["module"],
        div![
            C!["module-name"],
            div![
                "Paramètre"
            ]
        ],
        div![
            C!["module-content"],
            div![
                C!["parameter-select"],
                div![
                    C!["select"],
                    vec!["1", "2", "3"].iter().map(|x| {
                        vec![
                            input![
                                C!["button-parameter"],
                                attrs!{
                                    At::Type => "radio",
                                    At::Name => "speed",
                                    At::Id => x,
                                },
                                {
                                    let x = x.clone();
                                    ev(Ev::Input, move |_| {
                                        Msg::UpdateSpeed(parameter::Speed::from_str(x).unwrap())
                                    })
                                }
                            ],
                            label![
                                C!["label-parameter", "label-speed"],
                                attrs!{
                                    At::For => x
                                }
                            ],
                        ]
                    }).flatten().collect::<Vec<Node<Msg>>>(),
                ],
                div![
                    C!["select"],
                    vec!["apple", "pinapple", "rice"].iter().map(|x| {
                        vec![
                            input![
                                C!["button-parameter"],
                                attrs!{
                                    At::Type => "radio",
                                    At::Name => "apple",
                                    At::Id => x,
                                },
                                {
                                    let x = x.clone();
                                    ev(Ev::Input, move |_| {
                                        Msg::UpdateSkinApple(parameter::SkinApple::from_str(x).unwrap())
                                    })
                                }
                            ],
                            label![
                                C!["label-parameter", "label-apple"],
                                attrs!{
                                    At::For => x,
                                }
                            ]
                        ]
                    }).flatten().collect::<Vec<Node<Msg>>>(),
                ],
                div![
                    C!["select"],
                    vec!["snake", "chiken", "ladybeelt"].iter().map(|x| {
                        vec![
                            input![
                                C!["button-parameter"],
                                attrs!{
                                    At::Type => "radio",
                                    At::Name => "snake",
                                    At::Id => x,
                                },
                                {
                                    let x = x.clone();
                                    ev(Ev::Input, move |_| {
                                        Msg::UpdateSkinSnake(parameter::SkinSnake::from_str(x).unwrap())
                                    })
                                }
                            ],
                            label![
                                C!["label-parameter", "label-snake"],
                                attrs!{
                                    At::For => x,
                                }
                            ]
                        ]
                    }).flatten().collect::<Vec<Node<Msg>>>(),
                ],
            ],
            div![
                C!["center-play"],
                button![
                    attrs!["type" => "button"],
                    C!["button-play"],
                    ev(Ev::Click, |_| Msg::Play)
                ]
            ]
        ]
    ]
}


fn game(model: &Model) -> Node<Msg> {
    div![
        C!["game"],
        div![
            C!["info"],
            div![C!["apple"], div![format!("🍎 {}", model.apple)]],
            div![C!["score"], div![format!("Score :  {}", model.score)]],
            div![C!["time"], div![format!("{}", model.timer)]]
        ],
        div![
            C!["board"],
            div![
                C!["game-over", IF!(model.death => "game-over-animation")],
                // "Game Over",
                IF!(model.death => "GAME-OVER"),
            ],
            model
                .board
                .board
                .iter()
                .enumerate()
                .map(|(y, row)| {
                    div![
                        C!["row"],
                        row.iter()
                        .enumerate()
                            .map(|(x, tile)| {
                                {
                                    let css_tile = C![
                                        "tile",
                                        if (x + y % 2) % 2 == 0 { "even" } else { "odd" }
                                        ];
                                    match tile {
                                        Tile::Empty => div![css_tile],
                                        Tile::Apple => div![css_tile, C![model.skin_manager.get_skin_apple()]],
                                        Tile::Head(d) => div![
                                            css_tile,
                                            div![
                                                C![match d {
                                                    SimpleDirection::Up => model.skin_manager.get_skin_snake(1),
                                                    SimpleDirection::Down => model.skin_manager.get_skin_snake(0),
                                                    SimpleDirection::Left => model.skin_manager.get_skin_snake(3),
                                                    SimpleDirection::Right => model.skin_manager.get_skin_snake(2),
                                                }],
                                                div![]
                                            ]
                                        ],
                                        Tile::Queue(d) =>div![
                                            css_tile,
                                            div![
                                                C![match d {
                                                    SimpleDirection::Up => model.skin_manager.get_skin_snake(12),
                                                    SimpleDirection::Down => model.skin_manager.get_skin_snake(13),
                                                    SimpleDirection::Left => model.skin_manager.get_skin_snake(14),
                                                    SimpleDirection::Right => model.skin_manager.get_skin_snake(15),
                                                }]
                                            ]
                                        ],
                                        Tile::Body(d) => div![
                                            css_tile,
                                            div![
                                                C!["tile-color" match d {
                                                    ComplexDirection::Up => model.skin_manager.get_skin_snake(5),
                                                    ComplexDirection::Down => model.skin_manager.get_skin_snake(4),
                                                    ComplexDirection::Left => model.skin_manager.get_skin_snake(7),
                                                    ComplexDirection::Right => model.skin_manager.get_skin_snake(6),

                                                    ComplexDirection::UpLeft => model.skin_manager.get_skin_snake(10),
                                                    ComplexDirection::UpRight => model.skin_manager.get_skin_snake(11),
                                                    ComplexDirection::DownLeft => model.skin_manager.get_skin_snake(8),
                                                    ComplexDirection::DownRight => model.skin_manager.get_skin_snake(9),

                                                    ComplexDirection::LeftUp => model.skin_manager.get_skin_snake(9),
                                                    ComplexDirection::LeftDown => model.skin_manager.get_skin_snake(11),
                                                    ComplexDirection::RightUp => model.skin_manager.get_skin_snake(8),
                                                    ComplexDirection::RightDown => model.skin_manager.get_skin_snake(10),
                                                }]
                                            ]
                                        ]
                                    }
                                }
                            })
                            .collect::<Vec<Node<Msg>>>()
                    ]
                })
                .collect::<Vec<Node<Msg>>>()
        ]
    ]
}

fn record(model: &Model) -> Node<Msg> {
    div![
        C!["module"],
        div![
            C!["module-name"],
            "Record"
        ],
        div![
            C!["module-content"],
            ul![
                C!["list-record"]
            ]
        ]
    ]
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
