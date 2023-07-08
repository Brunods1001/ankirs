mod menus;
mod state;

use sqlx::SqlitePool;
use std::io::{self, Write};
use std::process::Command;

use crate::app::menus::traits::DecisionMaker;
use crate::app::state::AppState;

fn _clear_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "cls"])
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    } else {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        io::stdout().flush().unwrap();
    }
}

pub async fn start_app(pool: SqlitePool) -> Result<(), sqlx::Error> {
    println!("Starting app");

    let mut app_state = AppState::new();

    loop {
        let mut tx = pool.begin().await?;
        let (next_state, should_continue) = app_state.current_menu.make_decision(&mut tx).await?;
        app_state.navigate(next_state);

        if !should_continue {
            break;
        }

        tx.commit().await?;

        println!("State: {:?}", app_state);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_parse_input() {
        use crate::app::menus::{utils::parse_input, CardMenuOptions};
        assert_eq!(
            parse_input::<CardMenuOptions>("1"),
            Some(CardMenuOptions::Create)
        );
        assert_eq!(
            parse_input::<CardMenuOptions>("2"),
            Some(CardMenuOptions::List)
        );
        assert_eq!(
            parse_input::<CardMenuOptions>("3"),
            Some(CardMenuOptions::Update)
        );
        assert_eq!(
            parse_input::<CardMenuOptions>("4"),
            Some(CardMenuOptions::Delete)
        );
        assert_eq!(parse_input::<CardMenuOptions>("-1"), None);
    }
}
