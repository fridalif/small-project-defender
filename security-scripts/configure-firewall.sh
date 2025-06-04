#!/bin/sh

if [ "$(id -u)" -ne 0 ]; then
  echo "Этот скрипт должен запускаться с правами root" >&2
  exit 1
fi

if ! command -v ufw >/dev/null 2>&1; then
  echo "Установка ufw..."
  apt-get update >/dev/null 2>&1
  apt-get install -y ufw >/dev/null 2>&1
fi

echo "Сброс всех текущих правил ufw..."
ufw --force reset >/dev/null 2>&1

ufw default deny incoming >/dev/null 2>&1
ufw default allow outgoing >/dev/null 2>&1

is_valid_ip() {
  echo "$1" | grep -Eq '^([0-9]{1,3}\.){3}[0-9]{1,3}$' || return 1
  for octet in $(echo "$1" | tr '.' ' '); do
    [ "$octet" -le 255 ] || return 1
  done
  return 0
}

# Функция для проверки порта
is_valid_port() {
  echo "$1" | grep -Eq '^[0-9]+$' || return 1
  [ "$1" -le 65535 ] || return 1
  return 0
}

split_input() {
  echo "$1" | tr ' ' '\n' | grep -v '^$'
}

echo ""
echo "Введите IP-адреса, которым разрешен полный доступ (через пробел, или оставьте пустым):"
read allowed_ips_input

echo ""
echo "Введите номера портов, открытых для ВСЕХ (через пробел, например 22 80 443):"
read open_ports_input

echo ""
echo "Введите номера портов для ограничения доступа (через пробел):"
read restricted_ports_input

for ip in $(split_input "$allowed_ips_input"); do
  if is_valid_ip "$ip"; then
    echo "Разрешаем доступ с IP: $ip"
    ufw allow from "$ip" >/dev/null 2>&1
  else
    echo "Некорректный IP-адрес: $ip - пропускаем"
  fi
done

for port in $(split_input "$open_ports_input"); do
  if is_valid_port "$port"; then
    echo "Открываем порт $port для всех входящих соединений"
    ufw allow "$port" >/dev/null 2>&1
  else
    echo "Некорректный номер порта: $port - пропускаем"
  fi
done

for port in $(split_input "$restricted_ports_input"); do
  if is_valid_port "$port"; then
    echo ""
    echo "Настройка доступа к порту $port"
    
    # Ввод исключений для этого порта
    echo "Введите IP-адреса, которым разрешен доступ к порту $port (через пробел, или оставьте пустым):"
    read port_exceptions_input
    
    # Блокируем порт для всех
    ufw deny "$port" >/dev/null 2>&1
    
    # Разрешаем для исключений
    for ip in $(split_input "$port_exceptions_input"); do
      if is_valid_ip "$ip"; then
        echo "Разрешаем доступ к порту $port с IP: $ip"
        ufw allow from "$ip" to any port "$port" >/dev/null 2>&1
      else
        echo "Некорректный IP-адрес: $ip - пропускаем"
      fi
    done
  else
    echo "Некорректный номер порта: $port - пропускаем"
  fi
done

echo ""
echo "Активация правил ufw..."
ufw --force enable >/dev/null 2>&1

echo ""
echo "Текущие правила ufw:"
ufw status numbered

echo ""
echo "Настройка завершена!"
