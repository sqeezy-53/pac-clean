use std::error::Error;
use std::io;
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
    let app = AppConfig::new();
    app.clean_orphans()?;
    app.clean_pacman_cache()?;

    Ok(())
}


struct AppConfig {
    interactive: bool,
}
impl AppConfig {
    fn new() -> AppConfig {
        AppConfig {
            interactive: true,
        }
    }
    fn clean_orphans(&self) -> Result<(), Box<dyn Error>> {
        let output = Command::new("pacman").arg("-Qtdq").output()?;
        let stdout = String::from_utf8_lossy(&output.stdout);

        if stdout.trim().is_empty() {
            println!("Система чиста (сиротские пакеты не найдены).");
        } else {
            println!("Найдены следующие пакеты для удаления:\n{}", stdout);


            let should_delete = if self.interactive {
                println!("Удалить эти пакеты? [y/N]:");
                let mut answer = String::new();
                io::stdin().read_line(&mut answer)?;
                answer.trim().eq_ignore_ascii_case("y")
            } else {
                true
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