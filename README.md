<p align="center"><img src=".github/readme/banner.png" alt="UrodLauncher — Minecraft-лаунчер Города Уродов" width="1200"></p>

<p align="center"><strong>Сборки, контент и скины в одном лаунчере для Города Уродов и собственных миров.</strong></p>

<p align="center"><a href="README.en.md">English</a> · <a href="docs/FIGMA_IMPLEMENTATION_MAP.md">Дизайн из Figma</a> · <a href="docs/TEST-REPORT.md">Результаты тестов</a></p>

UrodLauncher **1.0.0 Stable** — официальный выпуск. [Что проверено и какие ограничения остаются](docs/STABLE_1.0.0_AUDIT.md).

## Скачать 1.0.0 Stable

<p align="center">
  <a href="https://github.com/Yozekkk/urod-launcher/releases/download/v1.0.0/UrodLauncher-Setup-1.0.0.exe"><img src=".github/readme/download-windows.png" alt="Windows — EXE" width="290"></a>
  <a href="https://github.com/Yozekkk/urod-launcher/releases/download/v1.0.0/UrodLauncher-1.0.0-amd64.deb"><img src=".github/readme/download-debian.png" alt="Debian, Ubuntu, Linux Mint — DEB" width="290"></a>
  <a href="https://github.com/Yozekkk/urod-launcher/releases/download/v1.0.0/UrodLauncher-1.0.0-x86_64.pkg.tar.zst"><img src=".github/readme/download-arch.png" alt="Arch, EndeavourOS, Manjaro — PKG" width="290"></a>
</p>

Кнопки скачивают установщики из [официального релиза v1.0.0](https://github.com/Yozekkk/urod-launcher/releases/tag/v1.0.0). Сверьте файл с [SHA256SUMS.txt](https://github.com/Yozekkk/urod-launcher/releases/download/v1.0.0/SHA256SUMS.txt). Пакеты не подписаны Authenticode; автоматическое обновление лаунчера отключено.

## Библиотека — стартовый экран

При каждом запуске открывается Библиотека. В ней можно создать сборку, выбрать Vanilla, Fabric, Forge, NeoForge или Quilt, установить Minecraft, запустить игру и остановить процесс. Поиск, сортировка, импорт и экспорт сборок доступны из этого же раздела.

<p align="center"><img src="docs/screenshots/readme/library.png" alt="Настоящее окно UrodLauncher: Библиотека с двумя тестовыми сборками" width="1000"></p>

<sub>Снимок 1280×720 сделан с работающего Tauri-приложения в изолированном тестовом профиле. Пустые обложки соответствуют сборкам без пользовательских иконок.</sub>

## Возможности

| Каталог | Аккаунты |
| --- | --- |
| <img src="docs/screenshots/readme/catalog.png" alt="Каталог Modrinth в UrodLauncher" width="520"> | <img src="docs/screenshots/readme/accounts.png" alt="Microsoft, Ely.by и тестовый Offline-аккаунт" width="520"> |
| Поиск модов, шейдеров, наборов ресурсов и сборок через Modrinth. Совместимость проверяется по версии Minecraft и загрузчику. | Microsoft, Ely.by и Offline. Можно хранить несколько профилей и переключаться между ними. Live-вход без разрешённых учётных данных не заявлен как проверенный. |

У выбранной сборки доступны управление установленным контентом, журнал запуска, настройки Java и памяти. Движок проверяет хеши загрузок, поддерживает отмену операций и восстановление там, где это предусмотрено архитектурой.

## Скины

<p align="center"><img src="docs/screenshots/readme/skins.png" alt="Библиотека скинов и 3D-просмотр в работающем UrodLauncher" width="1000"></p>

Добавьте PNG 64×64 или 64×32 размером до 1 МБ, выберите Classic/Steve либо Slim/Alex, поверните 3D-модель, переименуйте скин или замените текстуру. История применения хранится на диске отдельно для каждого аккаунта.

<details>
<summary>Посмотреть историю скинов</summary>
<p align="center"><img src="docs/screenshots/figma-rc/skin-history-860.png" alt="История скинов тестового аккаунта после перезапусков" width="700"></p>
</details>

Для Offline-аккаунта скин применяется локально в лаунчере. Для Microsoft нативный процесс отправляет PNG в Minecraft Services и проверяет ответ профиля; успешная работа с реальным аккаунтом пока не подтверждена. Загрузка скина на Ely.by из приложения недоступна.

## Настройки и кастомизация

| Настройки по Figma | Дополнительная кастомизация |
| --- | --- |
| <img src="docs/screenshots/readme/settings.png" alt="Глобальные настройки UrodLauncher по Figma" width="520"> | <img src="docs/screenshots/readme/customization.png" alt="Редактор кастомизации в работающем UrodLauncher" width="520"> |

Глобальные настройки включают обнаружение Java, параметры памяти и окна, управление ресурсами и лимит одновременных загрузок. Отдельные сборки могут переопределять Java и размер окна. В дополнительном редакторе доступны 37 элементов кастомизации; ещё три поля старой главной страницы читаются для совместимости с сохранёнными настройками. Значения сохраняются после перезапуска. Неподдерживаемые синхронизация, команды запуска и отдельные параметры честно отключены в интерфейсе.

## Установка

Скачайте пакет напрямую по кнопке выше. Файлы официального выпуска:

| Платформа | Файл | Команда |
| --- | --- | --- |
| Windows 10/11 x64 | <code>UrodLauncher-Setup-1.0.0.exe</code> | Запустите NSIS-установщик. Тихая установка проверяется на Windows runner; запуск GUI на Windows не проверен. |
| Debian / Ubuntu / Linux Mint x64 | <code>UrodLauncher-1.0.0-amd64.deb</code> | <code>sudo apt install ./UrodLauncher-1.0.0-amd64.deb</code> |
| Arch / EndeavourOS / Manjaro x64 | <code>UrodLauncher-1.0.0-x86_64.pkg.tar.zst</code> | <code>sudo pacman -U ./UrodLauncher-1.0.0-x86_64.pkg.tar.zst</code> |

Для сборки из исходников нужны Node.js, pnpm, Rust и системные библиотеки Tauri. На Linux требуется WebKitGTK 4.1. Инструкции: [DEVELOPMENT.md](docs/DEVELOPMENT.md) и [RELEASE.md](docs/RELEASE.md). Автообновление UrodLauncher безопасно отключено до появления собственного ключа подписи и endpoint.

## Частые вопросы

<details>
<summary>Нужна ли лицензия Minecraft для Offline-профиля?</summary>
<p>Offline-профиль подходит для одиночной игры и серверов, допускающих такой вход. Он не предоставляет лицензию Minecraft.</p>
</details>

<details>
<summary>Можно ли установить сборку сообщества?</summary>
<p>Проверенный manifest сборки «Города Уродов» пока не опубликован. Свои сборки создавать и запускать можно.</p>
</details>

<details>
<summary>Сохраняются ли аккаунты, скины и настройки?</summary>
<p>Да. Изолированный нативный тест прошёл 20 запусков с 10 Offline-аккаунтами, 15 скинами, раздельной историей и 40 сохранёнными значениями настроек. Live-сценарии Microsoft и Ely.by остаются в списке ограничений. Подробности: <a href="docs/TEST-REPORT.md">отчёт о тестировании</a>.</p>
</details>

## Документация и лицензия

[Карта переноса Figma](docs/FIGMA_IMPLEMENTATION_MAP.md) · [Дизайн-система](docs/DESIGN_SYSTEM.md) · [Тестирование](docs/TESTING.md) · [Аудит безопасности](docs/SECURITY_REVIEW.md) · [Происхождение графики](docs/GRAPHICS_AUDIT.md)

UrodLauncher основан на [NCreate Launcher](https://github.com/Yozekkk/ncreate-launcher) с модифицированными компонентами [Modrinth App](https://github.com/modrinth/code). Код распространяется по [GPL-3.0-only](LICENSE); обязательные сведения об авторах и лицензиях сохранены в [NOTICE.md](NOTICE.md) и [COPYING.md](COPYING.md). Проект не связан с Mojang или Microsoft.
