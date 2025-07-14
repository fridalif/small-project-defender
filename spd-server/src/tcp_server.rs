use crate::prelude::*;
use tokio::net::TcpListener;
use tokio::spawn;

Ваш код содержит несколько синтаксических и логических ошибок, которые предотвратят его корректную работу. Давайте разберём их пошагово, укажем проблемы и предложим исправленную версию.
Проблемы в коде

    Некорректный вызов server.accept:
        В строке let (mut socket, addr) = server.accept.await.unwrap_or_else(|_|=>continue) есть синтаксическая ошибка в синтаксисе |_|=>continue. Правильный синтаксис для unwrap_or_else должен включать блок или выражение, возвращающее значения, совместимые с ожидаемым типом (в данном случае (TcpStream, SocketAddr)). Использование continue в этом контексте некорректно, так как unwrap_or_else должно вернуть значение, а не управлять потоком цикла.
    Вызов tokio::spawn(serve):
        tokio::spawn ожидает футуру, но вы передаёте функцию serve без вызова. Кроме того, serve требует аргументы (socket, address, key), которые не передаются. Это приведёт к ошибке компиляции, так как serve — это не футура, а функция.
        Также tokio::spawn требует, чтобы переданная футура была 'static и реализовывала трейт Send. Это означает, что аргументы, передаваемые в serve, должны быть безопасно перемещаемы между потоками.
    Отсутствие обработки возвращаемого значения:
        server.accept возвращает Result<(TcpStream, SocketAddr), io::Error>, и хотя вы используете unwrap_or_else, текущая реализация не возвращает подходящее значение для продолжения работы цикла.
    Неопределённый prelude:
        В коде используется use crate::prelude::*, но неясно, что именно импортируется. Если prelude не содержит необходимых типов (например, tokio::net::TcpStream или std::sync::Arc), это может вызвать ошибки компиляции.
    Отсутствие возвращаемого значения в serve:
        Функция serve объявлена как async fn, но не возвращает Result или другой тип, что может быть проблемой, если вы планируете обрабатывать ошибки в асинхронном контексте.
    Неправильная работа с unwrap_or_else:
        Использование unwrap_or_else с continue синтаксически неверно, так как continue не является выражением, возвращающим значение. Кроме того, игнорирование ошибок с помощью continue может быть нежелательным, так как вы теряете информацию об ошибке.

Исправленный код

Ниже приведена исправленная версия кода с учётом указанных проблем. Предполагается, что AppConfig содержит поля server_settings с ip (тип String или &str) и port (тип u16). Также предполагается, что serve обрабатывает клиентское соединение.
rust
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

// Предполагаемая структура конфигурации
#[derive(Clone)]
pub struct AppConfig {
    pub server_settings: ServerSettings,
}

#[derive(Clone)]
pub struct ServerSettings {
    pub ip: String,
    pub port: u16,
}

async fn serve(mut socket: TcpStream, address: std::net::SocketAddr, key: String) {
    // Обработка клиентского соединения
}

pub async fn init_server(config: Arc<AppConfig>) -> std::io::Result<()> {
    let server = TcpListener::bind((config.server_settings.ip.as_str(), config.server_settings.port)).await?;
    println!("Сервер запущен на {}:{}", config.server_settings.ip, config.server_settings.port);

    loop {
        match server.accept().await {
            Ok((socket, addr)) => {
                println!("Новое соединение: {}", addr);
                tokio::spawn(serve(socket, addr, config.server_settings.init_secret.clone()));
            }
            Err(e) => {
                eprintln!("Ошибка при принятии соединения: {}", e);
                continue;
            }
        }
    }
}
