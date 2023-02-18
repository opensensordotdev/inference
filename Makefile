BUFDIR := common

all: protos sample-models

protos:
	git clone https://github.com/triton-inference-server/common
	cp -r common/protobuf/* ./proto
	rm -rf common

# Download triton sample models
#
# You have to manually upload these to your MinIO or S3-compatible endpoint in order for
# Triton to load them
sample-models:
	git clone \
  --depth 1  \
  --filter=blob:none  \
  https://github.com/triton-inference-server/server \
  ;
	rm -rf sample_models
	mkdir -p sample_models
	cp -r server/docs/examples/model_repository/simple* sample_models
	rm -rf server
	@echo "\nSample models written to sample_models. Upload to \nMinio Model Repository \
	to use with Triton"