SHELL = /bin/bash
# Разрешение кеширование в Docker
CACHE = yes
# Путь к файлам Docker
DOCKER_DIR = ci/docker
# Путь к CI скриптам
CI_SCRIPTS = ci/scripts

# Флаги сборки backend'a
GENERAL_BUILD_ARGS = --release
BACKEND_BUILD_ARGS = $(GENERAL_BUILD_ARGS) -p langbro


include .env

# Функция для установки флага о запрете кеширования
define is_need_to_use_cache
    if [ ! $(1) = yes ]; then\
		echo --no-cache ;\
    fi
endef

# Функция генерирующая базу для работы с docker
define base_docker_cmd
	echo docker-compose -f $(1)/docker-compose$(2).yml
endef

.PHONY:
	env \

	down-backend \
	build-backend \
	run-backend \
	count-backend \
	config-backend \


# Обновление файлов окружение в разных папках
env:
# 	Проверяю наличие главного .env файла 
	@if [ ! -e .env ]; then\
		echo .env file was not found && \
		exit 1 ;\
	fi

# 	Проверяю права файла env_init.sh
	@if [ ! $(shell stat -c %A $(CI_SCRIPTS)/env_init.sh) = -rwxrwxr-x ]; then\
		sudo chmod +x $(CI_SCRIPTS)/env_init.sh ;\
	fi
	
# 	Запускаю создание/обновление переменных окружения
	$(shell $(CI_SCRIPTS)/env_init.sh)


# #	Проверяю наличие файла languages_set.sh
# 	@if [ ! -e "$(CI_SCRIPTS)/languages_set.sh" ]; then\
# 		echo languages_set.sh file was not found && \
# 		exit 1 ;\
# 	fi

# # 	Проверяю права файла languages_set.sh
# 	@if [  ! $(shell stat -c %A $(CI_SCRIPTS)/languages_set.sh) = -rwxrwxr-x ]; then\
# 		sudo chmod +x $(CI_SCRIPTS)/languages_set.sh ;\
# 	fi

# 	Запускаю создание языкового файла
# 	$(shell $(CI_SCRIPTS)/languages_set.sh)


# Предварительный просмотр docker-compose файла
config-backend: env
	$(shell $(call base_docker_cmd, $(DOCKER_DIR),$(DOCKER_ENV))) config


# Удалить все volumes и сети созданые этим проектом
down-backend:	
	$(shell $(call base_docker_cmd, $(DOCKER_DIR),$(DOCKER_ENV))) down \
		--volumes \
		--remove-orphans


# Компиляция backend'a
build-backend:
	$(shell $(call base_docker_cmd, $(DOCKER_DIR),$(DOCKER_ENV))) build \
		--build-arg BUILD_ARGS="$(BACKEND_BUILD_ARGS)" \
		$(shell $(call is_need_to_use_cache, $(CACHE)))
	


# Запуск backend'a
run-backend: build-backend
	$(shell $(call base_docker_cmd, $(DOCKER_DIR),$(DOCKER_ENV))) up


# Подсчет кол-ва строк в проекте 
count-backend:
	find backend/src backend/tests -name langbro -prune -o -type f -name '*.rs' | xargs wc -l


.DEFAULT_GOAL := run-backend