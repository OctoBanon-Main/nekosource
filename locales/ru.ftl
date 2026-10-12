# Главный экран и элементы интерфейса
title-mock = 🐱 NekoSource
btn-refresh = Обновить
btn-clone = Клонировать
btn-settings = Настройки
btn-about = О программе
btn-close = Закрыть
btn-delete = Удалить выбранный
btn-commit = Опубликовать изменения
btn-save = Сохранить
btn-ok = ОК
btn-cancel = Отмена

# Метки и подсказки
label-url = URL:
hint-branch = ветка
label-local-repos = Локальные репозитории:
label-selected-repo = Выбран репозиторий:
label-commit-msg = Сообщение коммита:
label-username = Имя пользователя:
label-email = Email:
label-gpg = Подписывать коммиты через GPG
label-language = Язык:
label-dark-theme = Тёмная тема
label-gpg-id = ID GPG Ключа:
label-recent-commits = Недавние коммиты
label-changed-files = Изменённые файлы

placeholder-select = Выберите репозиторий из списка слева для управления изменениями
placeholder-no-commits = Коммитов пока нет
placeholder-no-changes = Рабочее дерево чистое

# Окно "Профиль" и "О программе"
settings-title = Настройки
about-title = О программе NekoSource
about-description = Спасибо за использование NekoSource! ^^
about-copyright = © 2026-н.в. OctoBanon
about-development = Разработка:
contributor-octobanon = - Главный разработчик
contributor-shadowcj = - Украинский перевод и белорусский перевод
contributor-xelframe = - Белорусский перевод

# Состояния и статусы
status-loading = Выполнение
status-success = Успешно
status-error = Ошибка
status-cloning = Клонирование...
status-committing = Коммит изменений...
gpg-no-keys = Ключи не найдены
gpg-refresh-keys = Обновить список ключей

# Пароль SSH
passphrase-title = Пароль SSH
passphrase-prompt = Введите пароль для вашего SSH-ключа:
passphrase-hint = Оставьте пустым, если у ключа нет пароля

# Ошибки: SSH
err-ssh-cancelled = Ввод SSH passphrase отменён

# Ошибки: Git
err-git-dest-exists = Директория уже существует: '{ $path }'
err-git-open = Не удалось открыть репозиторий: { $detail }
err-git-clone = Ошибка клонирования: { $detail }
err-git-stage = Ошибка индексирования изменений: { $detail }
err-git-write-tree = Ошибка записи дерева индекса: { $detail }
err-git-find-tree = Ошибка поиска объекта дерева: { $detail }
err-git-signature = Ошибка создания подписи автора: { $detail }
err-git-commit = Ошибка создания коммита: { $detail }
err-git-commit-buffer = Ошибка создания буфера коммита: { $detail }
err-git-commit-encoding = Буфер коммита содержит невалидный UTF-8
err-git-signed-commit = Ошибка создания подписанного коммита: { $detail }
err-git-ref-update = Ошибка обновления ссылки ветки: { $detail }
err-git-head-set = Ошибка перемещения HEAD: { $detail }
err-git-push = Ошибка push: { $detail }
err-git-branch-unknown = Не удалось определить имя текущей ветки

# Ошибки: Git Config
err-git-config-not-found = Глобальный git config (~/.gitconfig) не найден: { $detail }
err-git-config-open = Не удалось открыть git config для записи: { $detail }
err-git-config-write-name = Ошибка записи user.name: { $detail }
err-git-config-write-email = Ошибка записи user.email: { $detail }

# Ошибки: GPG
err-gpg-launch = Не удалось запустить gpg (проверьте PATH): { $detail }
err-gpg-stdin = Ошибка записи в stdin GPG: { $detail }
err-gpg-wait = Ошибка ожидания процесса GPG: { $detail }
err-gpg-failure = GPG завершился с ошибкой: { $detail }
err-gpg-encoding = Вывод GPG содержит невалидный UTF-8: { $detail }
workspace-title = Ваше рабочее пространство Git
placeholder-no-repos = Репозиториев пока нет. Клонируйте первый с помощью строки сверху.
publish-hint = Все изменения будут включены в коммит и отправлены на сервер.
