CURDIR := $(abspath $(dir $(lastword $(MAKEFILE_LIST))))

.PHONY: \
	llvm

llvm:
	cmake \
	-S $(CURDIR)/llvm/llvm \
	-B $(CURDIR)/llvm-build/gnu/build-final \
	-G Ninja \
	-DCMAKE_INSTALL_PREFIX='$(CURDIR)/llvm-build/gnu/target-final' \
	-DCMAKE_BUILD_TYPE='Release' \
	-DLLVM_TARGETS_TO_BUILD='RISCV' \
	-DLLVM_ENABLE_PROJECTS='lld;clang;mlir' \
	-DCMAKE_OSX_DEPLOYMENT_TARGET='11.0' \
	-DLLVM_DEFAULT_TARGET_TRIPLE='riscv64-unknown-elf' \
	-DLLVM_BUILD_UTILS='Off' \
	-DLLVM_BUILD_TESTS='Off' \
	-DLLVM_INCLUDE_UTILS='Off' \
	-DLLVM_INCLUDE_TESTS='Off' \
	-DLLVM_BUILD_INSTRUMENTED_COVERAGE='Off' \
	-DPACKAGE_VENDOR='QiBlock' \
	-DCMAKE_BUILD_WITH_INSTALL_RPATH=1 \
	-DLLVM_BUILD_DOCS='Off' \
	-DLLVM_INCLUDE_DOCS='Off' \
	-DLLVM_INCLUDE_BENCHMARKS='Off' \
	-DLLVM_INCLUDE_EXAMPLES='Off' \
	-DLLVM_ENABLE_DOXYGEN='Off' \
	-DLLVM_ENABLE_SPHINX='Off' \
	-DLLVM_ENABLE_OCAMLDOC='Off' \
	-DLLVM_ENABLE_ZLIB='Off' \
	-DLLVM_ENABLE_ZSTD='Off' \
	-DLLVM_ENABLE_LIBXML2='Off' \
	-DLLVM_ENABLE_BINDINGS='Off' \
	-DLLVM_ENABLE_LIBEDIT='Off' \
	-DLLVM_ENABLE_LIBPFM='Off' \
	-DCMAKE_EXPORT_COMPILE_COMMANDS='On' \
	-DPython3_FIND_REGISTRY='LAST' \
	-DBUG_REPORT_URL='https://github.com/QiBlock/qic/issues' \
	-DCLANG_ENABLE_ARCMT='Off' \
	-DCLANG_ENABLE_STATIC_ANALYZER='Off' \
	-DLLVM_OPTIMIZED_TABLEGEN='On' \
	-DLLVM_BUILD_RUNTIME='Off' \
	-DLLVM_BUILD_RUNTIMES='Off' \
	-DLLVM_INCLUDE_RUNTIMES='Off' \
	-DLLVM_ENABLE_WERROR='Off' \
	-DLLVM_ENABLE_ASSERTIONS='On' \
	-DLLVM_ENABLE_RTTI='Off' \
	-DCMAKE_EXE_LINKER_FLAGS='-Wl,-no_warn_duplicate_libraries' \
	-DCMAKE_SHARED_LINKER_FLAGS='-Wl,-no_warn_duplicate_libraries'
	ninja -C $(CURDIR)/llvm-build/gnu/build-final install

llvm-rt:
	cmake \
	-S $(CURDIR)/llvm/compiler-rt \
	-B $(CURDIR)/llvm-build/gnu/build-compiler-rt \
	-G Ninja \
	-DCOMPILER_RT_BUILD_BUILTINS='On' \
	-DCOMPILER_RT_BUILD_LIBFUZZER='Off' \
	-DCOMPILER_RT_BUILD_MEMPROF='Off' \
	-DCOMPILER_RT_BUILD_PROFILE='Off' \
	-DCOMPILER_RT_BUILD_SANITIZERS='Off' \
	-DCOMPILER_RT_BUILD_XRAY='Off' \
	-DCOMPILER_RT_DEFAULT_TARGET_ONLY='On' \
	-DCOMPILER_RT_BAREMETAL_BUILD='On' \
	-DCMAKE_BUILD_WITH_INSTALL_RPATH=1 \
	-DCMAKE_EXPORT_COMPILE_COMMANDS='On' \
	-DCMAKE_SYSTEM_NAME='unknown' \
	-DCMAKE_C_COMPILER_TARGET='riscv64' \
	-DCMAKE_ASM_COMPILER_TARGET='riscv64' \
	-DCMAKE_CXX_COMPILER_TARGET='riscv64' \
	-DCMAKE_INSTALL_PREFIX='$(CURDIR)/llvm-build/gnu/target-final' \
	-DCMAKE_BUILD_TYPE='Release' \
	-DCOMPILER_RT_TEST_COMPILER='$(CURDIR)/llvm-build/gnu/target-final/bin/clang' \
	-DCMAKE_C_FLAGS='--target=riscv64 -march=rv64emac -mabi=lp64e -mcpu=generic-rv64 -nostdlib -nodefaultlibs' \
	-DCMAKE_ASM_FLAGS='--target=riscv64 -march=rv64emac -mabi=lp64e -mcpu=generic-rv64 -nostdlib -nodefaultlibs' \
	-DCMAKE_CXX_FLAGS='--target=riscv64 -march=rv64emac -mabi=lp64e -mcpu=generic-rv64 -nostdlib -nodefaultlibs' \
	-DCMAKE_C_COMPILER='$(CURDIR)/llvm-build/gnu/target-final/bin/clang' \
	-DCMAKE_ASM_COMPILER='$(CURDIR)/llvm-build/gnu/target-final/bin/clang' \
	-DCMAKE_CXX_COMPILER='$(CURDIR)/llvm-build/gnu/target-final/bin/clang++' \
	-DCMAKE_AR='$(CURDIR)/llvm-build/gnu/target-final/bin/llvm-ar' \
	-DCMAKE_NM='$(CURDIR)/llvm-build/gnu/target-final/bin/llvm-nm' \
	-DCMAKE_RANLIB='$(CURDIR)/llvm-build/gnu/target-final/bin/llvm-ranlib' \
	-DLLVM_CMAKE_DIR='$(CURDIR)/llvm-build/gnu/target-final/lib/cmake/llvm' \
	-DLLVM_DEFAULT_TARGET_TRIPLE='riscv64-unknown-elf'
	cmake \
	--build $(CURDIR)/llvm-build/gnu/build-compiler-rt \
	--target install \
	--config Release

toolchain: llvm llvm-rt
