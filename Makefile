SHELL = /bin/bash
# Разрешение кеширование в Docker
CACHE = yes
# Путь к файлам Docker
DOCKER_DIR = ci/docker

# Флаги сборки backend'a
GENERAL_BUILD_ARGS = --release
BACKEND_BUILD_ARGS = $(GENERAL_BUILD_ARGS) -p langbro


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
	down-backend \
	build-backend \
	run-backend \
	count-backend \
	config-backend \


# Предварительный просмотр docker-compose файла
config-backend:
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


count-backend:
	find backend/src backend/tests -name langbro -prune -o -type f -name '*.rs' | xargs wc -l


.DEFAULT_GOAL := run-backend