
add_executable(solarsuite Main.cpp)
target_link_libraries(solarsuite PRIVATE lua microtar libzstd)

if (BUILD_SHARED_LIBS)
    target_link_libraries(solarsuite PRIVATE libcurl_shared)
else ()
    target_link_libraries(solarsuite PRIVATE libcurl_static)
endif ()
