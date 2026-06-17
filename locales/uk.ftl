# Головний екран та елементи інтерфейсу
title-mock = 🐱 NekoSource
btn-refresh = 🔄 Оновити
btn-clone = 📥 Клонувати
btn-settings = ⚙ Налаштування
btn-about = ℹ Про програму
btn-close = Закрити
btn-delete = 🗑 Видалити вибране
btn-commit = 🚀 Опублікувати зміни
btn-save = Зберегти
btn-ok = ОК
btn-cancel = Скасувати

# Мітки та підказки
label-url = URL:
hint-branch = branch
label-local-repos = Локальні репозиторії:
label-selected-repo = Вибраний репозиторій:
label-commit-msg = Повідомлення коміту:
label-username = Ім'я користувача:
label-email = Email:
label-gpg = 🔒 Підписувати коміти через GPG
label-gpg-id = ID GPG Ключа:
label-recent-commits = Недавні коміти

placeholder-select = Виберіть репозиторій зі списку зліва для керування змінами
placeholder-no-commits = Поки що немає комітів

# Вікно "Профіль" та "Про програму"
settings-title = ⚙ Налаштування користувача Git
about-title = ℹ Про програму NekoSource
about-description = Дякуємо за користування NekoSource! ^^
about-copyright = © 2026-н.ч. OctoBanon
about-development = Розробка:
contributor-octobanon = — Lead developer
contributor-shadowcj = — Український переклад & білоруський переклад :3
contributor-xelframe = — Білоруський переклад

# Стан та статуси
status-cloning = Клонування...
status-committing = Коміт...
gpg-no-keys = Ключів не знайдено
gpg-refresh-keys = Оновити список ключів

# Парольна фраза SSH
passphrase-title = Парольна фраза SSH
passphrase-prompt = Введіть парольну фразу для вашого ключа SSH:
passphrase-hint = Залиште порожнім, якщо ключ не має пароля

# Помилки: SSH
err-ssh-cancelled = Введення парольної фрази SSH скасовано

# Помилки: Git
err-git-dest-exists = Директорія вже існує: '{ $path }'
err-git-open = Не вдалося відкрити репозиторій: { $detail }
err-git-clone = Помилка клонування: { $detail }
err-git-stage = Не вдалося додати зміни до індексу: { $detail }
err-git-write-tree = Не вдалося записати дерево індексу: { $detail }
err-git-find-tree = Не вдалося знайти об'єкт дерева: { $detail }
err-git-signature = Не вдалося створити підпис автора: { $detail }
err-git-commit = Не вдалося створити коміт: { $detail }
err-git-commit-buffer = Не вдалося створити буфер коміту: { $detail }
err-git-commit-encoding = Буфер коміту містить некоректний UTF-8
err-git-signed-commit = Не вдалося створити підписаний коміт: { $detail }
err-git-ref-update = Не вдалося оновити посилання гілки: { $detail }
err-git-head-set = Не вдалося перемістити HEAD: { $detail }
err-git-push = Помилка push: { $detail }
err-git-branch-unknown = Не вдалося визначити назву поточної гілки

# Помилки: Git Config
err-git-config-not-found = Глобальний конфіг git (~/.gitconfig) не знайдено: { $detail }
err-git-config-open = Не вдалося відкрити конфіг git для запису: { $detail }
err-git-config-write-name = Не вдалося записати user.name: { $detail }
err-git-config-write-email = Не вдалося записати user.email: { $detail }

# Помилки: GPG
err-gpg-launch = Не вдалося запустити GPG (перевірте PATH): { $detail }
err-gpg-stdin = Не вдалося записати дані в stdin GPG: { $detail }
err-gpg-wait = Помилка під час очікування процесу GPG: { $detail }
err-gpg-failure = GPG завершився з помилкою: { $detail }
err-gpg-encoding = Вивід GPG містить некоректний UTF-8: { $detail }
