ifndef MICRO_TAG
MICRO_TAG = v0.1.1
endif

ifndef MICRO_URL
MICRO_URL = registry.cn-beijing.aliyuncs.com/imind/micro
endif

docker-micro:
	docker build -f micro/Dockerfile -t $(MICRO_URL):$(MICRO_TAG) .

.PHONY: docker-micro
