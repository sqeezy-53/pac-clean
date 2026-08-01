use std::error::Error;
use std::process::Command;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {


    let output = Command::new("pacman")
    .arg("-Qtdq")
    .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    if stdout.trim().is_empty() {
        println!("Система чиста");
    } else {
        println!("Найдены следующие пакеты для удаления:\n{}", stdout);
        println!("Удалить эти пакеты? [y/N]:");
        let mut answer = String::new();
        io::stdin().read_line(&mut answer)?;

        if answer.trim().eq_ignore_ascii_case("y") {
            println!("Начинаю удаление...");

            let status = Command::new("sudo")
            .arg("pacman")
            .arg("-Rns")
                .args(stdout.split_whitespace())
            .status()?;


            if status.success() {
                println!("Все ненужные пакеты удалены!")

            }else {
                println!("Ошибка")
            }
        }else {
            println!("Отмена.");
        }
    }
    Ok(())
}
