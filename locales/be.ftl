# Главный экран и элементы интерфейса
title-mock = 🐱 NekoSource
btn-refresh = Абнавіць
btn-clone = Кланіраваць
btn-settings = Налады
btn-about = Пра праграму
btn-close = Закрыць
btn-delete = Выдаліць выбранае
btn-commit = Апублікаваць змены
btn-save = Захаваць
btn-ok = ОК
btn-cancel = Скасаваць

# Метки и подсказки
label-url = URL:
hint-branch = branch
label-local-repos = Лакальныя рэпазіторыі:
label-selected-repo = Выбраны рэпазіторый:
label-commit-msg = Паведамленне каміту:
label-username = Імя карыстальніка:
label-email = Email:
label-gpg = Падпісваць каміты праз GPG
label-language = Мова:
label-dark-theme = Цёмная тэма
label-gpg-id = ID GPG-ключа:
label-recent-commits = Нядаўнія каміты
label-changed-files = Змененыя файлы

placeholder-select = Выберыце рэпазіторый са спісу злева каб кіраваць зменамі
placeholder-no-commits = Камітаў пакуль няма
placeholder-no-changes = Працоўнае дрэва чыстае

# Окно "Профиль" и "О программе"
settings-title = Налады
about-title = Пра праграму NekoSource
about-description = Дзякуй за выкарыстанне NekoSource! ^^
about-copyright = © 2026–н.ч. OctoBanon
about-development = Распрацоўка:
contributor-octobanon = - Lead developer
contributor-shadowcj = - Украінскі пераклад & беларускі пераклад
contributor-xelframe = - Беларускі пераклад

# Состояния и статусы
status-loading = Выкананне
status-success = Паспяхова
status-error = Памылка
status-cloning = Кланіраванне...
status-committing = Каміт...
gpg-no-keys = Ключоў не знойдзена
gpg-refresh-keys = Абнавіць спіс ключоў

# Пароль SSH
passphrase-title = Парольная фраза SSH
passphrase-prompt = Увядзіце парольную фразу для вашага ключа SSH:
passphrase-hint = Пакіньце пустым, калі ключ не мае пароля

# Ошибки: SSH
err-ssh-cancelled = SSH: Увод парольнай фразы SSH скасаваны

# Ошибки: Git
err-git-dest-exists = Дырэкторыя ўжо існуе: '{ $path }'
err-git-open = Не ўдалося адкрыць рэпазіторый: { $detail }
err-git-clone = Памылка кланіравання: { $detail }
err-git-stage = Памылка індэксавання змен: { $detail }
err-git-write-tree = Памылка запісу dрэва індэкса: { $detail }
err-git-find-tree = Памылка пошуку аб'екта дрэва: { $detail }
err-git-signature = Памылка стварэння подпісу аўтара: { $detail }
err-git-commit = Памылка стварэння каміту: { $detail }
err-git-commit-buffer = Памылка стварэння буфера каміту: { $detail }
err-git-commit-encoding = Буфер каміту змяшчае некарэктны UTF-8
err-git-signed-commit = Памылка стварэння падпісанага каміту: { $detail }
err-git-ref-update = Памылка абнаўлення спасылкі галіны: { $detail }
err-git-head-set = Памылка перамяшчэння HEAD: { $detail }
err-git-push = Памылка push: { $detail }
err-git-branch-unknown = Не ўдалося вызначыць імя бягучай галіны

# Ошибки: Git Config
err-git-config-not-found = Глабальны git config (~/.gitconfig) не знойдзены: { $detail }
err-git-config-open = Не ўдалося адкрыць git config для запісу: { $detail }
err-git-config-write-name = Памылка запісу user.name: { $detail }
err-git-config-write-email = Памылка запісу user.email: { $detail }

# Ошибки: GPG
err-gpg-launch = Не ўдалося запусціць GPG (праверце PATH): { $detail }
err-gpg-stdin = Памылка запісу ў stdin GPG: { $detail }
err-gpg-wait = Памылка чакання працэсу GPG: { $detail }
err-gpg-failure = GPG завяршыўся з памылкай: { $detail }
err-gpg-encoding = Вывад GPG змяшчае некарэктны UTF-8: { $detail }
workspace-title = Ваша працоўная прастора Git
placeholder-no-repos = Рэпазіторыяў пакуль няма. Клануйце першы праз радок уверсе.
publish-hint = Усе змены будуць уключаны ў коміт і адпраўлены на сервер.
