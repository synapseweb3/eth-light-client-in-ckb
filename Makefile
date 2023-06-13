MOLC := moleculec
MOLC_VERSION := 0.7.3

.PHONY: generate-protocols
GEN_MOL_IN_DIR := verification/schemas
GEN_MOL_OUT_DIR := verification/src/types/generated
GEN_MOL_FILES := ${GEN_MOL_OUT_DIR}/types.rs
generate-protocols: check-moleculec-version ${GEN_MOL_FILES}

${GEN_MOL_OUT_DIR}/%.rs: ${GEN_MOL_IN_DIR}/%.mol
	${MOLC} --language rust --schema-file $< | rustfmt > $@

.PHONY: check-moleculec-version
check-moleculec-version:
	test "$$(${MOLC} --version | awk '{ print $$2  }' | tr -d ' ')" = ${MOLC_VERSION}
