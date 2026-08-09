use std::error::Error;
use std::io;
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
    let mode = CleanMode::select_from_menu();
    let app = AppConfig::new(mode);
    app.clean_orphans()?;
    app.clean_pacman_cache()?;

    Ok(())
}
impl CleanMode {
    fn select_from_menu() -> CleanMode {
        println!("=== Select Cleaning Mode ===");
        println!("1. Interactive (ask before deletion) [Recommended]");
        println!("2. Force (delete immediately)");
        println!("3. DryRun (preview changes only)");
        println!("Enter choice [1-3] (default 1): ");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                CleanMode::Interactive
            },
            "2" => {
                CleanMode::Force
            },
            "3" => {
                CleanMode::DryRun
            },
            _ => {
                CleanMode::Interactive
            }
        }
    }
}


#[derive(Clone, Copy)]
enum CleanMode {
    Interactive,
    Force,
    DryRun,
}
struct AppConfig {
    mode: CleanMode,
}
impl AppConfig {
    fn new(mode: CleanMode) -> AppConfig {
        AppConfig { mode }
    }
    fn clean_orphans(&self) -> Result<(), Box<dyn Error>> {
        let output = Command::new("pacman").arg("-Qtdq").output()?;
        let stdout = String::from_utf8_lossy(&output.stdout);

        if stdout.trim().is_empty() {
            println!("Система чиста.");
        } else {
            println!("Найдены следующие пакеты для удаления:\n{}", stdout);


            let should_delete = match self.mode {
                CleanMode::Interactive => {
                    println!("Удалить эти пакеты? [y/N]:");
                    let mut answer = String::new();
                    io::stdin().read_line(&mut answer)?;
                    answer.trim().eq_ignore_ascii_case("y")
                },
                CleanMode::Force => {
                    println!("Удаляю пакеты");
                    true
                },
                CleanMode::DryRun => {
                    println!("Режим теста: ничего не будет удалено.");
                    false
                },
            };

            if should_delete {
                println!("Начинаю удаление...");
                let status = Command::new("sudo")
                    .arg("pacman")
                    .arg("-Rns")
                    .args(stdout.split_whitespace())
                    .status()?;

                if status.success() {
                    println!("Все ненужные пакеты удалены!");
                } else {
                    println!("Ошибка при удалении пакетов.");
                }
            } else {
                println!("Отмена.");
            }
        }
        Ok(())
    }
    fn clean_pacman_cache(&self) -> Result<(), Box<dyn Error>> {
        let output = Command::new("sudo").arg("paccache").arg("-r").output()?;
        let stdout = String::from_utf8_lossy(&output.stdout);

        if stdout.contains("no candidate") {
            println!("Кэш пакетов pacman уже чист (0 MiB).");
        } else {
            println!("{}", stdout.trim());
        }

        Ok(())
    }
}