@echo off
setlocal
cd /D "%~dp0"

d:\rafx\target\debug\rafx-shader-processor --trace ^
--glsl-path glsl ^
--spv-path processed_shaders ^
--metal-generated-src-path processed_shaders ^
--gles2-generated-src-path processed_shaders ^
--gles3-generated-src-path processed_shaders