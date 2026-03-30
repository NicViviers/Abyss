# cmake/arm-toolchain.cmake
set(CMAKE_SYSTEM_NAME Generic)

# Path to your arm-none-eabi toolchain executables (adjust if needed)
set(CMAKE_C_COMPILER   /usr/bin/arm-none-eabi-gcc)
set(CMAKE_CXX_COMPILER /usr/bin/arm-none-eabi-g++)
set(CMAKE_ASM_COMPILER /usr/bin/arm-none-eabi-gcc)

# Initial flags used for CMake compiler checks (must include cpu/fpu)
# These _INIT variables are used by CMake when it probes the compiler.
set(CPU_FLAGS "-mcpu=cortex-m33 -mthumb -mfpu=fpv5-sp-d16 -mfloat-abi=hard")

set(CMAKE_C_FLAGS_INIT   "${CPU_FLAGS}")
set(CMAKE_CXX_FLAGS_INIT "${CPU_FLAGS}")
set(CMAKE_EXE_LINKER_FLAGS_INIT "${CPU_FLAGS}")
