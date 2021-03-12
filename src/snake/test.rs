
#[allow(unused_imports)]

use super::*;

#[test]
#[ignore]
fn init() {
    println!("------------------------------------------- INIT ------------------------------------------------");
    let mut b = Board::new();
    b.update_board();
    println!("{:?}", b);
}

#[test]
#[ignore]
fn run_10_time_grow() {
    println!("------------------------------------------- GROW ------------------------------------------------");
    let mut b = Board::new();
    b.update_board();
    println!("{:?}", b);
    for _ in 0..10 {
        b.grow_snake();
        b.update_board();
        println!("{:?}", b);
    }
}

#[test]
#[ignore]
fn run_10_time_go_forward() {
    println!("------------------------------------------- GO ------------------------------------------------");
    let mut b = Board::new();
    b.update_board();
    println!("{:?}", b);
    for _ in 0..10 {
        b.grow_snake();
        b.move_queue();
        b.update_board();
        println!("{:?}", b);
    }
}

#[test]
fn boucle_add_unsetable_direction() {
    println!("------------------------------------------- BOUCLE + UNSETABLE ------------------------------------------------");
    let mut b = Board::new();
    b.update_board();
    println!("{:?}", b);
    for _ in 0..3 {
        b.grow_snake();
        b.update_board();
        println!("{:?}", b);
    }
    b.set_direction(SimpleDirection::Up);
    for _ in 0..5 {
        b.grow_snake();
        b.move_queue();
        b.update_board();
        println!("{:?}", b);
    }
    b.set_direction(SimpleDirection::Left);
    for _ in 0..5 {
        b.grow_snake();
        b.move_queue();
        b.update_board();
        println!("{:?}", b);
    }
    b.set_direction(SimpleDirection::Down);
    for _ in 0..5 {
        b.grow_snake();
        b.move_queue();
        b.update_board();
        println!("{:?}", b);
    }
    b.set_direction(SimpleDirection::Right);
    for _ in 0..5 {
        b.grow_snake();
        b.move_queue();
        b.update_board();
        println!("{:?}", b);
    }
    println!("***********************");
    b.set_direction(SimpleDirection::Left);
    b.grow_snake();
    b.move_queue();
    b.update_board();
    println!("{:?}", b);
}

#[test]
fn little_boucle() {
    println!("------------------------------------------- LITTLE BOUCLE ------------------------------------------------");
    let mut b = Board::new();
    b.update_board();
    println!("{:?}", b);
    for _ in 0..3 {
        b.grow_snake();
        b.update_board();
        println!("{:?}", b);
    }
    b.set_direction(SimpleDirection::Up);
    b.grow_snake();
    b.move_queue();
    b.update_board();
    println!("{:?}", b);
    b.set_direction(SimpleDirection::Left);
    b.grow_snake();
    b.move_queue();
    b.update_board();
    println!("{:?}", b);
    b.set_direction(SimpleDirection::Down);
    b.grow_snake();
    b.move_queue();
    b.update_board();
    println!("{:?}", b);
    b.set_direction(SimpleDirection::Right);
    b.grow_snake();
    b.move_queue();
    b.update_board();
    println!("{:?}", b);

    b.set_direction(SimpleDirection::Down);
    b.grow_snake();
    b.move_queue();
    b.update_board();
    println!("{:?}", b);
    b.set_direction(SimpleDirection::Left);
    b.grow_snake();
    b.move_queue();
    b.update_board();
    println!("{:?}", b);
    b.set_direction(SimpleDirection::Up);
    b.grow_snake();
    b.move_queue();
    b.update_board();
    println!("{:?}", b);
    b.set_direction(SimpleDirection::Right);
    b.grow_snake();
    b.move_queue();
    b.update_board();
    println!("{:?}", b);
}

#[test]
#[ignore]
fn eat() {
    println!("------------------------------------------- EAT ------------------------------------------------");
    let mut b = Board::new();
    b.update_board();
    println!("{:?}", b);
    for _ in 0..10 {
        b.grow_snake();
        b.move_queue();
        b.update_board();
        println!("{:?}", b);
    }
    b.set_direction(SimpleDirection::Up);
    for _ in 0..12 {
        if let Some(x) = b.grow_snake() {
            if x {
                println!("+---+\n|EAT|\n+---+");
                b.grow_snake();
                b.generate_apple()
            }
        };
        b.move_queue();
        b.update_board();
        println!("{:?}", b);
    }
}

#[test]
#[ignore]
fn dead_wall1() {
    println!("------------------------------------------- WALL 1 ------------------------------------------------");
    let mut b = Board::new();
    b.update_board();
    println!("{:?}", b);
    for _ in 0..15 {
        if let Some(_) = b.grow_snake() {
        } else {
            println!("+----+\n|DEAD|\n+----+")
        };
        b.move_queue();
        b.update_board();
        println!("{:?}", b);
    }
}

#[test]
#[ignore]
fn dead_wall2() {
    println!("------------------------------------------- WALL 2 ------------------------------------------------");
    let mut b = Board::new();
    b.update_board();
    b.set_direction(SimpleDirection::Up);
    b.set_direction(SimpleDirection::Left);
    println!("{:?}", b);
    for _ in 0..15 {
        if let Some(_) = b.grow_snake() {
        } else {
            println!("+----+\n|DEAD|\n+----+")
        };
        b.move_queue();
        b.update_board();
        println!("{:?}", b);
    }
}

#[test]
#[ignore]
fn dead_himself() {
    println!("------------------------------------------- HIMSELF ------------------------------------------------");
    let mut b = Board::new();
    b.update_board();
    println!("{:?}", b);
    for _ in 0..8 {
        b.grow_snake();
        b.update_board();
        println!("{:?}", b);
    }
    b.set_direction(SimpleDirection::Up);
    for _ in 0..2 {
        b.grow_snake();
        b.move_queue();
        b.update_board();
        println!("{:?}", b);
    }
    b.set_direction(SimpleDirection::Left);
    for _ in 0..2 {
        b.grow_snake();
        b.move_queue();
        b.update_board();
        println!("{:?}", b);
    }
    b.set_direction(SimpleDirection::Down);
    for _ in 0..5 {
        if let Some(_) = b.grow_snake() {
        } else {
            println!("+----+\n|DEAD|\n+----+")
        };
        b.move_queue();
        b.update_board();
        println!("{:?}", b);
    }
}

#[test]
#[ignore]
fn buffer() {
    println!("------------------------------------------- BUFFER ------------------------------------------------");
    let mut b = Board::new();
    b.update_board();
    println!("{:?}", b);
    for _ in 0..3 {
        b.grow_snake();
        b.update_board();
        println!("{:?}", b);
    }
    b.set_direction(SimpleDirection::Up);
    for _ in 0..5 {
        b.grow_snake();
        b.move_queue();
        b.update_board();
        println!("{:?}", b);
    }
    b.set_direction(SimpleDirection::Left);
    b.set_direction(SimpleDirection::Down);
    for _ in 0..3 {
        b.grow_snake();
        b.move_queue();
        b.update_board();
        println!("{:?}", b);
    }
    b.set_direction(SimpleDirection::Right);
    b.set_direction(SimpleDirection::Up);
    b.grow_snake();
    b.move_queue();
    b.update_board();
    println!("{:?}", b);
    b.set_direction(SimpleDirection::Right);
    for _ in 0..3 {
        b.grow_snake();
        b.move_queue();
        b.update_board();
        println!("{:?}", b);
    }
    b.set_direction(SimpleDirection::Down);
    b.set_direction(SimpleDirection::Left);
    b.set_direction(SimpleDirection::Down);
    b.set_direction(SimpleDirection::Left);
    for _ in 0..4 {
        b.grow_snake();
        b.move_queue();
        b.update_board();
        println!("{:?}", b);
    }
}
