use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
    Frame, Terminal,
};

fn main() -> Result<(), Box<dyn Error>> {

    async {

        // token can be given to any egg_mode method that asks for a token
        // user_id and screen_name refer to the user who signed in
        let con_token = egg_mode::KeyPair::new("consumer key", "consumer secret");
        // "oob" is needed for PIN-based auth; see docs for `request_token` for more info
        let request_token = egg_mode::auth::request_token( &con_token, "oob")?;
        let auth_url = egg_mode::auth::authorize_url(&request_token);
        println!( "{}", auth_url ); 
        let mut line = String::new(); 
        // give auth_url to the user, they can sign in to Twitter and accept your app's permissions.
        // they'll receive a PIN in return, they need to give this to your application
        let verifier = std::io::stdin().read_line(&mut line )?; // verifier PIN 

        // note this consumes con_token; if you want to sign in multiple accounts, clone it here
        let (token, user_id, screen_name) =
            egg_mode::auth::access_token(con_token, &request_token, verifier)?;
    }; 

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let res = run_app(&mut terminal);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }


    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f))?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}


fn ui<B: Backend>(f: &mut Frame<B>) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(30),
                Constraint::Percentage(70),
            ]
            .as_ref(),
        )
        .split(f.size());

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage( 80 ),
                Constraint::Percentage( 20 ),
            ]
            .as_ref(),
        )
        .split(chunks[1] ); 


    let block = Block::default().title("DMs").borders(Borders::ALL);
    f.render_widget(block, chunks[0]);

    let block = Block::default().title("Messages").borders(Borders::ALL);
    f.render_widget(block, right_chunks[0]);
    
    let block = Block::default().title("--").borders(Borders::ALL);
    f.render_widget(block, right_chunks[1]);
}
