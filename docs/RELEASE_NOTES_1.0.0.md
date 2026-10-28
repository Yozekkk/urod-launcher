# UrodLauncher 1.0.0 Stable

UrodLauncher is a Minecraft launcher for the «Город Уродов» community and personal worlds. The new desktop interface follows the owner's Figma layouts and uses their original icons and bundled Geologica/Inter fonts. Library now opens first; the five sections are Library, Catalog, Accounts, Skins and Settings.

## Русский

### Что нового

- **Библиотека сборок:** создание, поиск, сортировка, импорт и экспорт; установка и запуск Vanilla, Fabric, Forge, NeoForge и Quilt. Управление содержимым сборки, журналом игры, Java и памятью.
- **Каталог Modrinth:** поиск модов, шейдеров и наборов ресурсов с фильтрацией по версии Minecraft и загрузчику; установка в выбранную сборку.
- **Аккаунты:** Offline, Microsoft и Ely.by, переключение профилей. Переименование Offline-профиля сохраняет его UUID.
- **Скины:** безопасный импорт PNG, 3D-просмотр, модели Classic/Slim, редактирование, переименование и история выбранных скинов на диске отдельно для каждого аккаунта.
- **Настройки и внешний вид:** экраны на основе макетов Figma, параметры Java, памяти и окна, ограничение одновременных загрузок, 37 видимых элементов кастомизации и совместимость с тремя прежними полями.

Исправлены переход со старой Главной на Библиотеку, сохранение истории скинов, замена проверенного файла без предварительного удаления, ряд ошибок компоновки и проблемы зависимостей, обнаруженные аудитом.

### Установка

| Система | Файл |
| --- | --- |
| Windows 10/11 x64 | `UrodLauncher-Setup-1.0.0.exe` — запустите NSIS-установщик |
| Debian / Ubuntu / Linux Mint x64 | `UrodLauncher-1.0.0-amd64.deb` — `sudo apt install ./UrodLauncher-1.0.0-amd64.deb` |
| Arch / EndeavourOS / Manjaro x64 | `UrodLauncher-1.0.0-x86_64.pkg.tar.zst` — `sudo pacman -U ./UrodLauncher-1.0.0-x86_64.pkg.tar.zst` |

Сверьте SHA-256 скачанного файла с `SHA256SUMS.txt` этого релиза. Исходный код приложен отдельным архивом и доступен через GitHub. Автообновление отключено: собственных ключей подписи и сервера обновлений пока нет.

### Известные ограничения

Реальный вход Microsoft и Ely.by и изменение скина на сервере Minecraft не проверены без разрешённых тестовых учётных записей; автоматические тесты используют mocks. Запуск GUI в Windows и запуск установленного DEB в Debian не проверены. Отдельные панели синхронизации, произвольных команд запуска, полноэкранного режима и очистки кеша представлены в интерфейсе, но отключены до появления безопасной реализации. Проверенного manifest официальной сборки сообщества пока нет. Точное визуальное сравнение всех дочерних элементов Figma не завершено. Подробности: `docs/STABLE_1.0.0_AUDIT.md` и `docs/TEST-REPORT.md`.

## English

### Highlights

- **Library:** create, search, sort, import and export packs; install and launch Vanilla, Fabric, Forge, NeoForge and Quilt. Manage installed content, game logs, Java and memory.
- **Modrinth catalog:** browse mods, shaders and resource packs with Minecraft-version and loader compatibility filters, then install them into a selected pack.
- **Accounts:** Offline, Microsoft and Ely.by profiles with switching. Renaming an Offline profile preserves its UUID.
- **Skins:** validated PNG imports, a rotatable 3D preview, Classic/Slim models, editing and renaming, and persistent per-account selection history.
- **Settings and appearance:** Figma-based panels for Java, memory, window dimensions and download concurrency; 37 visible customization controls plus three preserved legacy values.

Fixes include routing legacy Home links to Library, persistent skin history, verified-file replacement without first removing the old cache entry, layout defects and vulnerable dependencies found in the production audit.

### Install

| Platform | File |
| --- | --- |
| Windows 10/11 x64 | `UrodLauncher-Setup-1.0.0.exe` — run the NSIS installer |
| Debian / Ubuntu / Linux Mint x64 | `UrodLauncher-1.0.0-amd64.deb` — `sudo apt install ./UrodLauncher-1.0.0-amd64.deb` |
| Arch / EndeavourOS / Manjaro x64 | `UrodLauncher-1.0.0-x86_64.pkg.tar.zst` — `sudo pacman -U ./UrodLauncher-1.0.0-x86_64.pkg.tar.zst` |

Compare the download's SHA-256 with this release's `SHA256SUMS.txt`. A separate GPL source archive is attached, and GitHub provides source snapshots. In-app automatic updates are disabled until UrodLauncher has its own signing key and update endpoint.

### Known limitations

Live Microsoft/Ely.by sign-in and server-side Minecraft skin changes have not been tested without authorized accounts; automated tests use mocks. Windows GUI execution and installed Debian GUI execution remain untested. Synchronization, arbitrary launch commands, fullscreen and cache cleanup controls are visibly disabled pending safe implementations. No verified official community pack manifest is available. Exact visual comparison of all Figma child components remains incomplete. See `docs/STABLE_1.0.0_AUDIT.md` and `docs/TEST-REPORT.md` for precise test scope.

UrodLauncher is GPL-3.0-only and derived from NCreate Launcher and modified Modrinth App components. See `NOTICE.md` and `COPYING.md`. It is not affiliated with Mojang or Microsoft. The installers are not Authenticode or Tauri-updater signed.
