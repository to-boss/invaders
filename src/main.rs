use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use invaders::{
    enemy::{Enemies, Enemy},
    frame::{self, new_frame, Drawable},
    player::Player,
    render, NUM_COLS,
};
use std::{error::Error, io, sync::mpsc, thread, time::Duration};

fn main() -> Result<(), Box<dyn Error>> {
    // Setup

    // Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Render Loop
    // (seperate thread)
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_rx.recv() {
                Ok(frame) => frame,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    let mut player = Player::new();
    let mut enemies = Enemies::new();

    // Game Loop
    'gameloop: loop {
        // update game loop
        // each update takes a frame
        enemies.update();

        for shot in player.shots.iter_mut() {
            shot.update();
        }
        player.shots.retain(|shot| !shot.exploded());

        for shot in player.shots.iter() {}

        // init drawable frame
        let mut curr_frame = new_frame();

        // Input handling
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        break 'gameloop;
                    }
                    KeyCode::Char(' ') => player.shoot(),
                    KeyCode::Up => player.move_up(),
                    KeyCode::Down => player.move_down(),
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    _ => {}
                }
            }
        }

        // Draw and render
        player.draw(&mut curr_frame);
        enemies.draw(&mut curr_frame);
        for shot in player.shots.iter() {
            shot.draw(&mut curr_frame);
        }
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(10)); // 100 fps poggers
    }

    // Cleanup
    drop(render_tx);
    render_handle.join().unwrap();

    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
